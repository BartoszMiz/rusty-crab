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
		send_response(&mut stream, 405, "Method Not Allowed", vec![]);
	}

	let uri = get_uri(request_line);
	match fs::read(format!("{}{}", RESOURCE_DIR, uri)) {
		Err(_) => {
			send_response(&mut stream, 404, "Not Found", vec![]);
			warn!("{} not found!", &uri);
		},
		Ok(data) => {
			send_response(&mut stream, 200, "OK", data);
			info!("Serving URI: {}", &uri);
		}
	}

	stream.flush().unwrap();
	stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn get_uri(request_line: String) -> String {
	let uri_regex = Regex::new(r"^GET (/.*) HTTP/\d\.\d").unwrap();
	let uri = uri_regex.captures(&request_line).unwrap().get(1).unwrap().as_str();
	if uri == "/" {
		return "/index.html".to_string();
	}
	uri.to_string()
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
