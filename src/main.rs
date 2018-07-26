use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

struct Message {}

fn main() {
    let mut write_buf = String::new();

    let timeout_duration = Some(Duration::from_millis(10));
    let mut con = TcpStream::connect("irc.mozilla.org:6667").expect("unable to connect to server");
    con.set_nodelay(true).expect("set_nodelay has failed");
    con.set_read_timeout(timeout_duration)
        .expect("set_read_timeout has failed");

    send_user("testuser", &mut write_buf);
    send_nick("testuser123", &mut write_buf);

    loop {
        loop_once(&mut con, &mut write_buf);
    }
}

fn loop_once(con: &mut TcpStream, write_buf: &mut String) {
    let mut buf = [0; 1024];
    let messages = read_connection(con, &mut buf);
    handle_messages(&messages, write_buf);
    write_connection(con, write_buf);
}

fn read_connection(con: &mut TcpStream, buf: &mut [u8]) -> Vec<Message> {
    let result: Vec<Message> = vec![];

    match con.read(buf) {
        Ok(bytes_read) => {
            if bytes_read > 0 {
                println!(
                    "=={}==",
                    str::from_utf8(buf).expect("server sent us invalid utf8")
                );
            }
        }
        Err(_) => (),
    }
    result
}

fn handle_messages(messages: &[Message], write_buf: &String) {}

fn write_connection(con: &mut TcpStream, write_buf: &mut String) {
    if write_buf.len() == 0 {
        return;
    }

    println!("writting {}", write_buf);
    con.write_all(write_buf.as_bytes())
        .expect("could not write_all");
    write_buf.clear();
}

fn send_user(username: &str, write_buf: &mut String) {
    write_buf.push_str("USER ");
    write_buf.push_str(username);
    write_buf.push_str(" 0 * :");
    write_buf.push_str(username);
    write_buf.push('\r');
    write_buf.push('\n');
}

fn send_nick(nick: &str, write_buf: &mut String) {
    write_buf.push_str("NICK ");
    write_buf.push_str(nick);
    write_buf.push('\r');
    write_buf.push('\n');
}
