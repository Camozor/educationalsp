use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ClientRequest {
    pub id: Option<i32>,
    pub method: Method,
    pub params: Option<ClientRequestParams>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Method {
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "hover")]
    Hover,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ClientRequestParams {
    InitializeParams {},
    HoverParams { text_document: String },
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub id: i32,
    pub result: Option<InitializeResult>,
}

#[derive(Serialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
}

#[derive(Serialize)]
pub struct ServerCapabilities {}

pub fn deserialize_client_request(client_request: &str) -> ClientRequest {
    let result: ClientRequest =
        serde_json::from_str(client_request).expect("Could not deserialize client_request");
    return result;
}

pub fn serialize_response(response: &ResponseMessage) -> String {
    serde_json::to_string(response).expect("Could not serialize response")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_client_request() {
        let result = deserialize_client_request("{\"id\":1,\"method\": \"initialize\"}");
        assert_eq!(result.method, Method::Initialize);
    }
}
