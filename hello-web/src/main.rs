use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io;
use std::str::from_utf8;

fn main() {
	loop {
		let mut msg = String::new();
		 io::stdin()
		        .read_line(&mut msg)
		        .expect("Failed to read line");
        let msg = msg.as_bytes();
		match TcpStream::connect("localhost:3333") {
		    Ok(mut stream) => {
		        println!("Successfully connected to server in port 3333");

		        stream.write(msg).unwrap();
		        println!("Sent message \"{:?}\", awaiting reply", msg);

		        let mut data = [0 as u8; 50];
		        match stream.read_exact(&mut data) {
		            Ok(_) => {
		                if &data == msg{
		                    println!("Reply is ok!");
		                } else {
		                    let text = from_utf8(&data).unwrap();
		                    println!("Unexpected reply: {}", text);
		                }
		            },
		            Err(e) => {
		                println!("Failed to recieve data: {}", e);
		            }
		        }
		    },
		    Err(e) => {
		        println!("Failed to connect: {}", e);
		        break;
		    }
		}
	}
    println!("Terminated!");
}
