use std::io::Write;
use ecw::COOKIE_PATH;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;




fn main() {
    let mut flag_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(ecw::files::get(ecw::files::FLAG).expect("File should exist"))
        .expect("Error opening log file");
    flag_file.write_all(build_flag().as_bytes()).expect("Error writing to log file");

    let mut flag_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(COOKIE_PATH)
        .expect("Error opening log file");
    flag_file.write_all(build_cookie().as_bytes()).expect("Error writing to log file");
}

fn build_flag() -> String {
    format!("ECW{{iLikePlanes_{}}}", get_rand_string(10))
}

fn build_cookie() -> String {
    get_rand_string(10)
}

fn get_rand_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
