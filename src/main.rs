use std::io::Write;
use tokio::sync::mpsc;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(ecw::files::get(ecw::files::LOGS).expect("File should exist"))
        .expect("Error opening log file");
    file.write_all(b"").expect("Error writing to log file");
    let (tx, mut rx) = mpsc::channel(1);
    let join_handle = tokio::spawn(async { ecw::tcp_main(tx).await });
    rx.recv().await;
    ecw::make_admin_requests().await;
    join_handle.await.unwrap();
}
