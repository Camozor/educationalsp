use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRequest {
    pub method: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMessage {
    pub id: i32,
    pub result: Option<ResponseResult>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseResult {
    pub capabilities: ServerCapabilities,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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
        let result = deserialize_client_request("{\"method\": \"initialize\"}");
        assert_eq!(result.method, "initialize");
    }
}
