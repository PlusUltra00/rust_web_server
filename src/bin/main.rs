extern crate web_server;
use web_server::ThreadPool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::fs::File;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
	let pool = ThreadPool::new(4);
	for stream in listener.incoming().take(2) {
		let stream = stream.unwrap();
		pool.execute(|| {
			handle_connection(stream);
		})
	}
	println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf = [0; 1024];
	stream.read(&mut buf).unwrap();
	
	let (status_line, filename) = if buf.starts_with(b"GET / HTTP/1.1\r\n") {
		("HTTP/1.1 200 \r\n\r\n", "src/index.html")
	}
	else if buf.starts_with(b"GET /sleep HTTP/1.1\r\n") {
		thread::sleep(Duration::from_secs(5));
		("HTTP/1.1 200 \r\n\r\n", "src/index.html")
	}
	else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/404.html")
	};

	let mut file = File::open(filename).unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	let response = format!("{} {}", status_line, contents);
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}