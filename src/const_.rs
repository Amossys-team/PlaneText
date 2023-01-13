use std::io::Read;

pub const TOKIO_PORT: u16 = 80;
pub const IV_SIZE: usize = 16;
pub const LOG_KEY: &[u8] = &[0x84, 2, 3, 0x84, 0xb5, 0xf6, 0xf7, 8, 0xa, 10, 0x11, 12, 0xfb, 0xfb, 15, 7];
pub const HEADER_NAME: &str = "cookie";
pub const MAX_NB_HEADERS: usize = 255;
pub const COOKIE_PATH: &str = "cookie";
pub const ACCUEIL: &str = "/welcome.html";

lazy_static::lazy_static! {
    pub static ref ADMIN_COOKIE: String = {
        let mut flag_file = std::fs::OpenOptions::new()
            .read(true)
            .open(COOKIE_PATH)
            .expect("Error opening log file");
        let mut buf = Vec::new();
        flag_file.read_to_end(&mut buf).expect("Unable to read flag file");
        String::from_utf8(buf).expect("Error with cookie format")
    };
}

pub mod files {
    pub const FOLDER: &str = "pages";
    pub const AVIONS: &str = "/planes";
    pub const FLAG: &str = "/flag.txt";
    pub const WELCOME: &str = "/";
    pub const GEN_ALEA: &str = "/randomgenerator.rs";
    pub const USELESS: &str = "/uselessfile_lol.html";
    pub const LOGS: &str = "/logs.txt";
    pub const LIST: [&str; 6] = [AVIONS, FLAG, WELCOME, GEN_ALEA, USELESS, LOGS];
    pub fn get(filename: &str) -> std::io::Result<String> {
        if !LIST.contains(&filename) && filename != super::ACCUEIL {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
        }
        Ok(format!("{}{}", FOLDER, filename))
    }
}
