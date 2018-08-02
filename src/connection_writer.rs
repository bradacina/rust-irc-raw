use std::io::Write;

pub fn write_connection(con: &mut Write, write_buf: &mut String) {
    if write_buf.len() == 0 {
        return;
    }

    println!("writting {}", write_buf);
    con.write_all(write_buf.as_bytes())
        .expect("could not write_all");
    write_buf.clear();
}