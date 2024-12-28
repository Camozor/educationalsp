use core::str;

pub fn get_message(buffer: &[u8], length: usize) -> &str {
    let truncated_buffer = &buffer[..length];
    match str::from_utf8(&truncated_buffer) {
        Ok(message_str) => message_str,
        Err(e) => panic!("{}", e),
    }
}

pub fn get_content_length(msg: &str) -> i32 {
    let splitted: Vec<&str> = msg.split("\r\n\r\n").collect();

    let content_length_header = splitted[0];

    let skip_part_length = "Content-Length: ".len();
    let length = &content_length_header[skip_part_length..];

    length.parse().unwrap()
}

pub fn get_content(msg: &str) -> &str {
    let splitted: Vec<&str> = msg.split("\r\n\r\n").collect();
    splitted[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(
            get_content_length("Content-Length: 16\r\n\r\n{\"Testing\":true}"),
            16
        );
    }
}
