use std::net::TcpStream;
use std::io::Read;
use std::str;
use std::time::Duration;

struct Message {

}

fn main() {

    let timeout_duration = Some(Duration::from_millis(10));
    let mut con = TcpStream::connect("irc.mozilla.org:6667").expect("unable to connect to server");
    con.set_nodelay(true).expect("set_nodelay has failed");
    con.set_read_timeout(timeout_duration).expect("set_read_timeout has failed");

    loop {
        loop_once(&mut con);
    }
}

fn loop_once(con: &mut TcpStream) {

    let mut buf = [0; 1024];
    let messages = read_connection(con, &mut buf);
    handle_messages(&messages);
    write_connection();
}

fn read_connection(con: &mut TcpStream, buf: &mut [u8])-> Vec<Message> {
    let result: Vec<Message> = vec![];

    match con.read(buf) {
        Ok(bytes_read) => println!("In read_connection read {} bytes: {}", bytes_read, str::from_utf8(buf).unwrap()),
        Err(_) => ()
    }
    result
}

fn handle_messages(messages: &[Message]) {}

fn write_connection() {
}