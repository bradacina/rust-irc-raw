use connection_reader::ConnectionReader;
use connection_writer as cw;
use message_handler as mh;
use std::io::stdout;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use terminal::{color, cursor, misc, term_init};
use widgets::window::Window;

mod connection_reader;
mod connection_writer;
mod message_handler;
mod terminal;
mod widgets;

extern crate libc;

fn main() {
    let mut _term_init = term_init::TermInit::init();

    misc::use_alternate_screen_buffer();

    let mut w1 = Window::new(20, 20, 1, 0);
    let mut w2 = Window::new(20, 20, 1, 20);

    w1.add("we are here\r\n");
    w1.add("somewhere over the rainbow\r\n");
    w1.add("a\r\n");
    w1.add("b\n");
    w1.add("c\n");

    w2.add("there was a boy\r\nwho had a dog\nand bingo was his name\r\nq\r\nw\ne");

    w1.draw();
    w2.draw();
    stdout().flush();

    std::thread::sleep(std::time::Duration::from_secs(5));

    w1.add("we are here\r\n");
    w1.add("somewhere over the rainbow\r\n");
    w1.add("a\r\n");
    w1.add("b\n");
    w1.add("c\n");
    w2.add("there was a boy\r\nwho had a dog\nand bingo was his name\r\nq\r\nw\ne");
    

    w1.draw();
    w2.draw();
    stdout().flush();

    std::thread::sleep(std::time::Duration::from_secs(5));

    color::reset();

    misc::use_main_screen_buffer();
    /*
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
    */
}

fn loop_once(con: &mut TcpStream, write_buf: &mut String, reader: &mut ConnectionReader) {
    let message_iter = reader.read_messages(con);

    if let Some(messages) = message_iter {
        mh::handle_messages(messages, write_buf);
    }
    cw::write_connection(con, write_buf);
}
