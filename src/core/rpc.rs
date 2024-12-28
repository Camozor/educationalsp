use core::str;

use log::{error, info};

use crate::core::serialization::deserialize_client_request;

use super::serialization::{serialize_response, ClientRequest, ResponseMessage};

pub fn decode(buffer: &[u8], size: usize) -> ClientRequest {
    let message = get_message(&buffer, size);
    let content = get_content(message);

    if get_content_length(&message) != i32::try_from(content.len()).unwrap() {
        error!("Message content does not match Content-Length header");
    }

    info!("Content: {}", content);

    deserialize_client_request(content)
}

fn get_message(buffer: &[u8], length: usize) -> &str {
    let truncated_buffer = &buffer[..length];
    match str::from_utf8(&truncated_buffer) {
        Ok(message_str) => message_str,
        Err(e) => panic!("{}", e),
    }
}

fn get_content_length(msg: &str) -> i32 {
    let splitted: Vec<&str> = msg.split("\r\n\r\n").collect();

    let content_length_header = splitted[0];

    let skip_part_length = "Content-Length: ".len();
    let length = &content_length_header[skip_part_length..];

    length.parse().unwrap()
}

fn get_content(msg: &str) -> &str {
    let splitted: Vec<&str> = msg.split("\r\n\r\n").collect();
    splitted[1]
}

pub fn encode(response: &ResponseMessage) -> String {
    let serialized_response = serialize_response(&response);
    "".to_string();

    let content_length = serialized_response.bytes().len();

    format!(
        "Content-Length: {}\r\n\r\n{}",
        content_length, serialized_response
    )
}

#[cfg(test)]
mod tests {
    use crate::core::serialization::{ResponseResult, ServerCapabilities};

    use super::*;

    #[test]
    fn test_get_content_length() {
        assert_eq!(
            get_content_length("Content-Length: 16\r\n\r\n{\"Testing\":true}"),
            16
        );
    }

    #[test]
    fn test_encode() {
        let response = ResponseMessage {
            id: 1,
            result: Some(ResponseResult {
                capabilities: ServerCapabilities {},
            }),
        };
        let encoded = encode(&response);
        assert_eq!(
            encoded,
            "Content-Length: 37\r\n\r\n{\"id\":1,\"result\":{\"capabilities\":{}}}"
        );
    }
}
