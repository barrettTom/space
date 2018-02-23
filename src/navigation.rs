use std::net::TcpStream;
use std::io::BufRead;
use std::io::BufReader;

pub fn Navigation(mut stream :TcpStream, mut buff_r : BufReader<TcpStream>){
    loop {
        let mut data = String::new();
        buff_r.read_line(&mut data).unwrap();
    }
}
