use core::str;

use log::{error, info};

use crate::core::serialization::{
    deserialize_generic_client_request, HoverClientRequest, InitializeClientRequest,
    InitializeResult, InitializedClientRequest, ServerCapabilities,
};

use super::serialization::{serialize_response, ClientRequest, Method, ResponseMessage};

pub fn decode(buffer: &[u8], size: usize) -> Box<dyn ClientRequest> {
    let message = get_message(&buffer, size);
    let client_request = get_content(message);

    if get_content_length(&message) != i32::try_from(client_request.len()).unwrap() {
        error!("Message content does not match Content-Length header");
    }

    info!("Content: {}", client_request);

    let generic_request = deserialize_generic_client_request(client_request);

    match generic_request.method {
        Method::Initialize => {
            Box::new(InitializeClientRequest::deserialize_request(client_request))
        }
        Method::Initialized => Box::new(InitializedClientRequest::deserialize_request(
            client_request,
        )),
        Method::Hover => Box::new(HoverClientRequest::deserialize_request(client_request)),
    }
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

pub fn handle_request_initialize(client_request: &InitializeClientRequest) {
    info!("Handling initialize");
    let response = ResponseMessage {
        id: client_request.id.expect("Initialize message must have id"),
        result: Some(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(true),
            },
        }),
    };
    send_response(&response);
}

pub fn handle_request_initialized() {
    info!("Handling initialized");
}

pub fn handle_request_hover(client_request: &HoverClientRequest) {
    info!("Handling hover");
    info!("{:?}", client_request);
}

fn send_response(response: &ResponseMessage) {
    let encoded_response = encode(&response);
    println!("{}", encoded_response);
    info!("Response sent: {}", encoded_response);
}

#[cfg(test)]
mod tests {
    use crate::core::serialization::{InitializeResult, ServerCapabilities};

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
            result: Some(InitializeResult {
                capabilities: ServerCapabilities {
                    hover_provider: None,
                },
            }),
        };
        let encoded = encode(&response);

        let expected_encoded =
            "Content-Length: 57\r\n\r\n{\"id\":1,\"result\":{\"capabilities\":{\"hoverProvider\":null}}}";
        assert_eq!(encoded, expected_encoded);
    }
}
