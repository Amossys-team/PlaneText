use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

use crate::{update_iv, Error};

pub async fn tcp_main(tx: mpsc::Sender<u8>) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", crate::TOKIO_PORT))
        .await
        .unwrap_or_else(|_| panic!("Unable to bind to port {}", crate::TOKIO_PORT));
    let counter = Arc::new(Mutex::new(0));
    tx.send(0).await.expect("Problem with channel");
    loop {
        let acceptation = listener.accept().await;
        if let Err(e) = acceptation {
            println!("Unexpected error accepting TCP connection: {:?}", e);
            continue;
        }
        let (stream, _) = acceptation.expect("Already handled error");
        let local_counter = counter.clone();
        tokio::spawn(async { handle_connection(stream, local_counter).await });
    }
}

async fn handle_connection(mut client_stream: TcpStream, counter: Arc<Mutex<usize>>) {
    let mutex = &*counter;
    let counter = get_current_and_increase(mutex).expect("Unexpected mutex error");
    let mut iv = crate::update_iv(&(counter as u64).to_be_bytes());
    loop {
        let mut buffer = Vec::new();
        if read_stream(&mut client_stream, &mut buffer).await.is_err() {
            return;
        }

        let reply = build_reply(&buffer, iv).await;
        if reply.is_err() {
            client_stream
                .write_all(b"Unexpected error. Not part of the chellenge")
                .await
                .map_err(|e| panic!("Error in proxy while sending to client: {e}"))
                .unwrap();
        }
        let reply = reply.unwrap();
        let err = client_stream.write_all(&reply).await;
        if let Err(e) = err {
            if e.kind() != std::io::ErrorKind::BrokenPipe {
                panic!("Error in proxy while sending to client: {e}")
            } else {
                return;
            }
        }
        iv = update_iv(&iv);
    }
}

async fn build_reply(buffer: &[u8], iv: [u8; crate::IV_SIZE]) -> Result<Vec<u8>, http::Error> {
    let encrypted_req = get_encrypted_req(buffer, iv).expect("Error encrypting request");
    let hex_iv = &bytes_to_hex(&iv).expect("Error writing hex string");
    let hex_request = &bytes_to_hex(&encrypted_req).expect("Error writing hex string");
    let http_result = crate::handle_http(buffer);
    crate::build_reply(http_result, hex_iv, hex_request)
}

fn get_encrypted_req(buffer: &[u8], iv: [u8; crate::IV_SIZE]) -> Result<Vec<u8>, Error> {
    crate::aes_cbc_cipher(crate::LOG_KEY, &iv, buffer)
}

fn bytes_to_hex(buffer: &[u8]) -> Result<String, Error> {
    use core::fmt::Write;
    let mut buffer_hex = String::with_capacity(2 * buffer.len());

    for byte in buffer {
        write!(buffer_hex, "{:02x}", byte).or_else(|_| Err(Error::Write))?;
    }
    Ok(buffer_hex)
}

pub async fn read_stream(stream: &mut TcpStream, buf: &mut Vec<u8>) -> Result<(), Error> {
    stream.readable().await.map_err(|_| Error::Read)?;
    loop {
        let mut temp = [0u8; 4096];
        let nb_read = match stream.try_read(&mut temp) {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Ok(());
            }
            Err(_) => {
                return Err(Error::Read);
            }
            Ok(0) => {
                return Ok(());
            }
            Ok(n) => n,
        };
        buf.append(&mut temp[..nb_read].to_vec());
    }
}

fn get_current_and_increase(
    counter: &Mutex<usize>,
) -> std::result::Result<usize, PoisonError<MutexGuard<usize>>> {
    let mut counter = counter.lock()?;
    let current_counter: usize = *counter;
    *counter += 1;
    Ok(current_counter)
}
