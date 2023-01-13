FROM rust:latest

WORKDIR /tmp
COPY . .
RUN chmod +x run.sh
RUN cargo build --quiet --release 
CMD ["sh", "-c", "/tmp/run.sh"]
