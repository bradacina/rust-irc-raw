use std::io::Read;

pub struct ConnectionReader {
    buf: [u8; 1024],
    prefix: String,
}

impl ConnectionReader {
    pub fn new() -> ConnectionReader {
        return ConnectionReader {
            buf: [0; 1024],
            prefix: String::with_capacity(1024),
        };
    }

    pub fn read_messages(&mut self, con: &mut Read) -> Option<Vec<String>> {

        let bytes_read = match con.read(&mut self.buf) {
            Ok(br) => br,
            Err(_) => 0,
        };

        if bytes_read == 0 {
            return None;
        }

        let message_result = self.extract_leftover(bytes_read);

        if message_result.is_none() {
            return None;
        }

        let message = message_result.unwrap();

        //println!("Received\r\n{}", message);

        Some(message.lines().map(|item| String::from(item)).collect::<Vec<_>>())
    }

    fn extract_leftover(&mut self, bytes_read: usize) -> Option<String> {
        let mut s = String::from_utf8(self.buf[0..bytes_read].to_vec())
            .expect("could not convert buffer sent from server to utf8");

        if self.prefix.len() > 0 {
            s.insert_str(0, &self.prefix);
            self.prefix.clear();
        }

        let find_result = s.rfind("\r\n");

        if let Some(last_index) = find_result {
            if last_index < s.len() - 2 {
                self.prefix.push_str(&s[last_index + 2..]);
                s.truncate(last_index);
            } else {
                self.prefix.clear();
            }

            return Some(s);
        } else {
            self.prefix.push_str(&s);

            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionReader;

    #[test]
    fn test_prefix_gets_set() {
        let mut cr = ConnectionReader::new();
        cr.prefix = String::from("Why ");
        String::from("hello\r\nthere")
            .chars()
            .enumerate()
            .for_each(|(i, v)| cr.buf[i] = v as u8);
        let msg = cr.extract_leftover(12);
        assert_eq!(msg, Some(String::from("Why hello")));
        assert_eq!(cr.prefix, String::from("there"));
    }

    #[test]
    fn test_prefix_gets_set_on_full_buffer() {
        let mut cr = ConnectionReader::new();

        let buflen = cr.buf.len();
        let halfbl = buflen / 2;

        cr.prefix = String::from("111");
        for i in 0..buflen {
            cr.buf[i] = 't' as u8;
        }

        cr.buf[halfbl - 1] = 'm' as u8; // set marker at the end of string
        cr.buf[halfbl] = '\r' as u8;
        cr.buf[halfbl + 1] = '\n' as u8;

        let msg_opt = cr.extract_leftover(buflen);
        assert!(msg_opt.is_some());
        let msg = msg_opt.unwrap();
        assert_eq!(msg.len(), halfbl + 3);
        assert_eq!(msg[(halfbl + 2)..(halfbl + 3)].chars().nth(0), Some('m'));
        assert!(msg[0..3].chars().all(|c| c == '1'));
        assert_eq!(cr.prefix.len(), halfbl - 2);
    }
}
