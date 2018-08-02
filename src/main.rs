use connection_reader::ConnectionReader;
use connection_writer as cw;
use message_handler as mh;
use std::net::TcpStream;
use std::time::Duration;

mod connection_reader;
mod connection_writer;
mod message_handler;

fn main() {
    let mut write_buf = String::new();

    let timeout_duration = Some(Duration::from_millis(10));
    let mut con = TcpStream::connect("irc.mozilla.org:6667").expect("unable to connect to server");
    con.set_nodelay(true).expect("set_nodelay has failed");
    con.set_read_timeout(timeout_duration)
        .expect("set_read_timeout has failed");

    let mut reader = ConnectionReader::new();

    mh::send_user("testuser", &mut write_buf);
    mh::send_nick("testuser123", &mut write_buf);

    loop {
        loop_once(&mut con, &mut write_buf, &mut reader);
    }
}

fn loop_once(con: &mut TcpStream, write_buf: &mut String, reader: &mut ConnectionReader) {
    let message_iter = reader.read_messages(con);

    if let Some(messages) = message_iter {
        mh::handle_messages(messages, write_buf);
    }
    cw::write_connection(con, write_buf);
}
