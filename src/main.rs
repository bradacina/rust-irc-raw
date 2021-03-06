#[macro_use]
extern crate failure;

use connection_reader::ConnectionReader;
use connection_writer as cw;
use message_handler as mh;
use program_lifecycle::ProgramLifecycle;
use std::io::{stdout, Write};
use std::net::TcpStream;
use std::time::Duration;
use terminal::misc;
use widgets::window::Window;

mod connection_reader;
mod connection_writer;
mod message_handler;
mod program_lifecycle;
mod terminal;
mod widgets;

extern crate libc;

fn main() {
    let mut _program_init = ProgramLifecycle::init();

    let (x, y) = misc::query_cursor_pos().expect("Error while trying to Query Cursor Position");

    //break up the screen width into 3 areas (60% for main display, 20% for users in channel, 20% for channel list )

    let main_window_size: usize = (x as f32 * 0.6) as usize;
    let users_window_size: usize = (x as f32 * 0.2) as usize;
    let chanlist_window_size: usize = (x as f32 * 0.2) as usize;

    let mut main_window = Window::new(main_window_size as u16, (y - 2) as u16, 1, 1);

    let mut users_window = Window::new(
        users_window_size as u16,
        y as u16,
        1,
        (main_window_size + 1) as u16,
    );

    let mut chanlist_window = Window::new(
        chanlist_window_size as u16,
        y as u16,
        1,
        (main_window_size + users_window_size + 1) as u16,
    );

    main_window.add("main window\n");
    users_window.add("users window\n");
    chanlist_window.add("channels window\n");

    main_window.auto_scroll = true;

    main_window.draw();
    users_window.draw();
    chanlist_window.draw();

    stdout().flush().unwrap();

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
        let message_iter = reader.read_messages(&mut con);

        if let Some(messages) = message_iter {
            for message in messages {
                main_window.add(&message);
                main_window.add("\n");
                mh::handle_message(&message, &mut write_buf)
                    .expect(&format!("{} was not in the correct format", message));
            }

            cw::write_connection(&mut con, &mut write_buf);

            main_window.draw();
            stdout().flush().unwrap();
        }
    }
}
