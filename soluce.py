import hashlib
import socket
import string
from collections.abc import Generator
from typing import Dict, Optional, Tuple, Callable

SERVER_ADDRESS = ("192.168.200.217", 80)
ALPHABET = string.printable


class File:
    A = b"planes"
    FLAG = b"flag.txt"
    WELCOME = b""
    LOGS = b"logs.txt"
    USELESS = b"uselessfile_lol.html"
    GENERATOR = b"randomgenerator.rs"


def request(filename: bytes, cookie: bytes) -> bytes:
    end = b"\r\n"
    return b"GET /" + filename + b" HTTP/1.1" + end + b"cookie: " + cookie + 2 * end


def build_iv(counter: int) -> bytes:
    return hashlib.sha256(counter.to_bytes(8, "big")).digest()[:16]


def update_iv(iv: bytes) -> bytes:
    return hashlib.sha256(iv).digest()[:16]


def xor(bb1: bytes, bb2: bytes) -> bytes:
    if len(bb1) != len(bb2):
        raise Exception
    return b"".join([(b1 ^ b2).to_bytes(1, "big") for b1, b2 in zip(bb1, bb2)])


def bruteforce_two_chars() -> Generator[bytes, None, None]:
    return (bytes(a + b, "utf-8") for a in ALPHABET for b in ALPHABET)


def send_and_receive(s: socket.socket, payload: bytes) -> bytes:
    s.sendall(payload)
    return s.recv(1024)


def recover_iv(server_reply: bytes) -> bytes:
    return bytes.fromhex(str(server_reply[34:66])[2:-1])


def recover_secondblock(server_reply: bytes) -> bytes:
    return server_reply[100:132]


def recover_thirdblock(server_reply: bytes) -> bytes:
    return server_reply[132:164]


def recover_firstblock(server_reply: bytes) -> bytes:
    return server_reply[68:100]


def get_ivs_and_blocks() -> Tuple[Dict[str, bytes], Dict[str, bytes]]:
    with socket.socket() as s:
        s.connect(SERVER_ADDRESS)
        s.sendall(request(File.LOGS, b""))
        received = s.recv(1024)
        admin_ivs = {
            file: bytes.fromhex(iv.decode("utf-8"))
            for file, iv in {
                "avions": received[169:201],
                "flag": received[300:332],
                "/": received[431:463],
                "generator": received[562:594],
                "useless": received[725:757],
            }.items()
        }

        admin_pertinent_blocks = {
            "avions": received[235:267],
            "flag": received[398:430],
            "/": received[497:529],
            "generator": received[660:692],
            "useless": received[823:855],
        }
        return (admin_ivs, admin_pertinent_blocks)


def block_recovery(block_number: int) -> Callable[[bytes], bytes]:
    if block_number not in [2, 3]:
        raise Exception
    if block_number == 2:
        return recover_secondblock
    return recover_thirdblock


def attack(
    file: bytes,
    admin_iv: bytes,
    admin_block: str,
    cookie_start: bytes,
    block_number: int,
) -> Optional[bytes]:
    with socket.socket() as s:
        s.connect(SERVER_ADDRESS)
        reply = send_and_receive(s, b"\r\n")
        iv = recover_iv(reply)
        for test in bruteforce_two_chars():
            iv = update_iv(iv)
            base_payload = request(file, cookie_start + test)
            trick = xor(admin_iv, iv)
            payload = xor(base_payload[:16], trick) + base_payload[16:]
            reply = send_and_receive(s, payload)
            block = block_recovery(block_number)
            if block(reply) == admin_block:
                return test
        return None


def main():
    admin_ivs, admin_pertinent_blocks = get_ivs_and_blocks()

    cookie_start = attack(
        File.A, admin_ivs["avions"], admin_pertinent_blocks["avions"], b"", 2
    )
    print(f"[*] Found first two characters: {cookie_start}")

    cookie_start += attack(
        File.USELESS,
        admin_ivs["useless"],
        admin_pertinent_blocks["useless"],
        cookie_start,
        3,
    )
    print(f"[*] Found first four characters: {cookie_start}")

    cookie_start += attack(
        File.GENERATOR,
        admin_ivs["generator"],
        admin_pertinent_blocks["generator"],
        cookie_start,
        3,
    )
    print(f"[*] Found first six characters: {cookie_start}")

    cookie_start += attack(
        File.WELCOME,
        admin_ivs["/"],
        admin_pertinent_blocks["/"],
        cookie_start,
        2,
    )
    print(f"[*] Found first height characters: {cookie_start}")

    cookie_start += attack(
        File.FLAG,
        admin_ivs["flag"],
        admin_pertinent_blocks["flag"],
        cookie_start,
        3,
    )
    print(f"[*] Found cookie: {cookie_start}")

    with socket.socket() as s:
        s.connect(SERVER_ADDRESS)
        flag_reply = send_and_receive(s, request(File.FLAG, cookie_start))
        flag_start = flag_reply.find(b'ECW{')
        flag = flag_reply[flag_start:]
        flag = str(flag)[2:-1]
        print(f"Flag: {flag}")


if __name__ == "__main__":
    main()
