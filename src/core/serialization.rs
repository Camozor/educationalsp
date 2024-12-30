use serde::{Deserialize, Serialize};

use super::rpc::{handle_request_hover, handle_request_initialize, handle_request_initialized};

pub trait ClientRequest {
    fn deserialize_request(raw_str: &str) -> Self
    where
        Self: Sized;

    fn handle(&self);
}

pub fn deserialize_generic_client_request(client_request: &str) -> GenericClientRequest {
    let result: GenericClientRequest =
        serde_json::from_str(client_request).expect("Could not deserialize client_request");
    return result;
}

#[derive(Deserialize, Debug)]
pub struct GenericClientRequest {
    pub method: Method,
}

#[derive(Deserialize, Debug)]
pub struct InitializeClientRequest {
    pub id: Option<i32>,
    pub params: Option<InitializeParams>,
}

impl ClientRequest for InitializeClientRequest {
    fn deserialize_request(raw_str: &str) -> Self {
        serde_json::from_str(raw_str).expect("Could not deserialize InitializeClientRequest")
    }

    fn handle(&self) {
        handle_request_initialize(&self);
    }
}

#[derive(Deserialize, Debug)]
pub struct InitializedClientRequest {
    pub id: Option<i32>,
    pub params: Option<InitializedParams>,
}

impl ClientRequest for InitializedClientRequest {
    fn deserialize_request(raw_str: &str) -> Self {
        serde_json::from_str(raw_str).expect("Could not deserialize InitializedClientRequest")
    }

    fn handle(&self) {
        handle_request_initialized();
    }
}

#[derive(Deserialize, Debug)]
pub struct HoverClientRequest {
    pub id: Option<i32>,
    pub params: Option<HoverParams>,
}

impl ClientRequest for HoverClientRequest {
    fn deserialize_request(raw_str: &str) -> Self {
        serde_json::from_str(raw_str).expect("Could not deserialize HoverClientRequest")
    }

    fn handle(&self) {
        handle_request_hover(&self);
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Method {
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "textDocument/hover")]
    Hover,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializedParams {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HoverParams {
    text_document: TextDocumentIdentifier,
    position: Position,
}

#[derive(Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Position {
    line: u32,
    character: u32,
}

pub trait ResponseMessage {
    fn serialize_message(&self) -> String;
}

#[derive(Serialize)]
pub struct InitializeResponseMessage {
    pub id: i32,
    pub result: Option<InitializeResult>,
}

impl ResponseMessage for InitializeResponseMessage {
    fn serialize_message(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize InitializeResponseMessage")
    }
}

#[derive(Serialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    pub hover_provider: Option<bool>,
}

#[derive(Serialize)]
pub struct HoverResponseMessage {
    pub id: i32,
    pub result: Option<HoverResult>,
}

impl ResponseMessage for HoverResponseMessage {
    fn serialize_message(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize HoverResponseMessage")
    }
}

#[derive(Serialize)]
pub struct HoverResult {
    pub contents: MarkupContent,
}

#[derive(Serialize)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

#[derive(Serialize)]
pub enum MarkupKind {
    Plaintext,
    Markdown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_generic_client_request() {
        let payload = r#"{"id": 1, "method": "textDocument/hover", "params": {"textDocument": "/home/user/hello.md"}}"#;
        let result = deserialize_generic_client_request(payload);
        assert_eq!(result.method, Method::Hover);
    }

    #[test]
    #[ignore = "temp"]
    fn test_deserialize_client_request_method_choosing() {
        let payload = r#"{"id": 1, 
            "method": "textDocument/hover", 
            "params": {"textDocument": {uri: "file:///home/user/hello.md"},
                        "position": {"line": 17, "character": 32}
            }}"#;
        let hover_request = HoverClientRequest::deserialize_request(payload);

        assert_eq!(
            hover_request.params.unwrap().text_document.uri,
            format!("file:///home/user/hello.md")
        );
    }
}
