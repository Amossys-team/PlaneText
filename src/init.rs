use std::io::Write;
use tokio::{net::TcpStream, io::AsyncWriteExt};

pub async fn make_admin_requests() {
    for filename in crate::files::LIST {
        send_and_log(&format!(
            "GET {} HTTP/1.1\r\n{}: {}\r\n\r\n",
            filename,
            crate::HEADER_NAME,
            *crate::ADMIN_COOKIE
        ))
        .await;
    }
}

async fn send_and_log(payload: &str) {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", crate::TOKIO_PORT))
        .await
        .expect("Error sending the first admin messages");
    stream
        .write_all(payload.as_bytes())
        .await
        .expect("Error sending the first admin messages");
    let mut buffer = Vec::new();
    crate::read_stream(&mut stream, &mut buffer)
        .await
        .expect("Error reading from server at startup");
    let mut headers = [httparse::EMPTY_HEADER; 10];
    let mut parser = httparse::Response::new(&mut headers);
    parser.parse(&buffer).unwrap();
    let iv = parser.headers[1].name;
    let encrypted = std::str::from_utf8(parser.headers[1].value).expect("Non str character in header");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(crate::files::get(crate::files::LOGS).expect("file should exist"))
        .expect("Error opening log file at startup");
    file.write_all(format!("{}: {}\n", iv, encrypted).as_bytes())
        .expect("Error writing to logfile at startup");
}
