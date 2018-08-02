pub fn handle_messages(messages: Vec<String>, write_buf: &mut String) {
    for message in messages {
        handle_message(&message, write_buf)
            .expect(&format!("{} was not in the correct format", message));
    }
}

pub fn send_user(username: &str, write_buf: &mut String) {
    write_buf.push_str("USER ");
    write_buf.push_str(username);
    write_buf.push_str(" 0 * :");
    write_buf.push_str(username);
    write_buf.push_str("\r\n");
}

pub fn send_nick(nick: &str, write_buf: &mut String) {
    write_buf.push_str("NICK ");
    write_buf.push_str(nick);
    write_buf.push_str("\r\n");
}

fn handle_message(message: &String, write_buf: &mut String) -> Result<(), String> {
    let mut token_iter = message.split_whitespace();

    let first = token_iter
        .next()
        .ok_or(format!("cannot split message {}", message))?;
    let cmd: &str;

    if first.eq("PING") {
        cmd = "PING";
    } else if first.starts_with(":") {
        cmd = token_iter
            .next()
            .ok_or(format!("no command found in message {}", message))?;
    } else {
        return Err(format!("message {} does not start with :", message));
    }

    let args = token_iter.collect::<Vec<_>>();

    handle_command(cmd, args, write_buf);

    Ok(())
}

fn handle_command(cmd: &str, args: Vec<&str>, write_buf: &mut String) {
    match cmd {
        "PING" => handle_ping(args, write_buf),
        _ => (),
    }
}

fn handle_ping(args: Vec<&str>, write_buf: &mut String) {
    write_buf.push_str("PONG ");
    write_buf.push_str(args[0]);
    write_buf.push_str("\r\n");
}
