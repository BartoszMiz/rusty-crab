use std::{net::TcpListener, io::Write};

const BIND_ADDRESS: &str = "127.0.0.1";
const BIND_PORT: u32 = 8080;

fn main() {
	let address = format!("{}:{}", BIND_ADDRESS, BIND_PORT);
	let tcp_listener = match TcpListener::bind(&address) {
		Ok(listener) => {
			println!("Server listening on {}", &address);
			listener
		}
		Err(e) => {
			println!("Failed to bind {} - {}", &address, e);
			return;
		}
	};
	
	for stream in tcp_listener.incoming() {
		match stream {
			Ok(mut stream) => {
				println!("Incoming connection from: {}", stream.peer_addr().unwrap());
				stream.write_all(b"HTTP/1.1 418 I'm a teapot\r\n").unwrap();
				stream.flush().unwrap();
				stream.shutdown(std::net::Shutdown::Both).unwrap();
			},
			Err(e) => {
				println!("Connection failed: {}", e);
			}
		}
	}
}
