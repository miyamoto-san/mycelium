mod lib;
use lib::{Node, State, Machine};
use std::io::prelude::Read;
use std::net::{TcpStream};

const IP_ADDR: &str = "127.0.0.1";
const PORT: u32 = 8080;

fn handle_client(mut stream: TcpStream, _state: &Machine) {
    let mut buff = [1; 1024];
    stream.read(&mut buff).unwrap();
    println!("stream: {}", String::from_utf8_lossy(&buff[..]));
}

fn main() {
    let machine: Machine = Machine::new(State {
        test: String::from("test"),
        test2: String::from("test2")
    });
    let node = Node::new(IP_ADDR, PORT, 4);
    loop {
        let (stream, sock_addr) = node.serve();
        println!("Incoming connection from: {}", sock_addr);
        node.execute(move || handle_client(stream, &machine));
    }
}
