use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use log::{debug, info, warn, error};
use regex::Regex;
use std::fs;

const BIND_ADDRESS: &str = "127.0.0.1";
const BIND_PORT: u32 = 8080;
const RESOURCE_DIR: &str = "www";

fn main() {
	env_logger::init();
	let address = format!("{}:{}", BIND_ADDRESS, BIND_PORT);
	let tcp_listener = match TcpListener::bind(&address) {
		Ok(listener) => {
			info!("Server listening on {}", &address);
			listener
		}
		Err(e) => {
			error!("Failed to bind {} - {}", &address, e);
			return;
		}
	};
	
	for stream in tcp_listener.incoming() {
		match stream {
			Ok(stream) => {
				info!("Incoming connection from: {}", stream.peer_addr().unwrap());
				handle_connection(stream);
			},
			Err(e) => {
				warn!("Connection failed: {}", e);
			}
		}
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0u8; 1024];
	stream.read(&mut buffer).unwrap();
	let request = String::from_utf8_lossy(&buffer).to_string();
	debug!("Request: \n{}", &request);

	let request_line: String = request.lines().take(1).collect();
	if !request_line.starts_with("GET") {
		send_empty_response(&mut stream, 405, "Method Not Allowed");
	}

	let uri_regex = Regex::new(r"^GET (/.*) HTTP/\d\.\d").unwrap();
	let mut uri = uri_regex.captures(&request_line).unwrap().get(1).unwrap().as_str();
	if uri == "/" {
		uri = "/index.html";
	}
	
	info!("Serving URI: {}", &uri);

	match fs::read(format!("{}{}", RESOURCE_DIR, uri)) {
		Err(_) => {
			send_empty_response(&mut stream, 404, "Not Found");
			warn!("{} not found!", &uri);
		},
		Ok(data) => {
			send_response(&mut stream, 200, "OK", data);
		}
	}

	stream.flush().unwrap();
	stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn send_response(stream: &mut TcpStream, code: u32, reason_phrase: &str, mut content: Vec<u8>) {
	let mut response = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n",
		code, 
		reason_phrase,
		content.len()
	).into_bytes();
	response.append(&mut content);
	stream.write_all(&response).unwrap();
}

fn send_empty_response(stream: &mut TcpStream, code: u32, reason_phrase: &str) {
	send_response(stream, code, reason_phrase, vec![]);
}
