# PlaneText

## Install 
### With `cargo`
```sh
cargo build --release
./target/release/flags && sudo ./target/release/ecw
```
`sudo` is only needed to access port 80.
Alternatively, the value can be changed in the line
```rust
pub const TOKIO_PORT: u16 = 80;
```
of `src/const_.rs`.

### With `Docker`
```
sudo docker build -t plane_text .      
sudo docker run -p 80:80 -it plane_text
```

## Instructions
The website of an airport enthusiast is available in an early version (on
localhost port 80). The administrator has watched a couple cybersecurity videos
on youtube and implemented logging of requests protected with encryption.
Recover his cookie to get the flag.

## Quick Write-up 
The script `soluce.py` solves the challenge (the variable
`SERVER_ADDRESS` must be set accordingly).    

The write-up is available at `https://gist.github.com/Amossys-team/e99cc3b979b30c047e6855337fec872e`.
