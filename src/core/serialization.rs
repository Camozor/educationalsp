use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRequest {
    pub method: String,
}

pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
}

pub struct ServerCapabilities {}

pub fn deserialize_client_request(client_request: &str) -> ClientRequest {
    let result: ClientRequest =
        serde_json::from_str(client_request).expect("Could not deserialize client_request");
    return result;
}

pub fn serialize_initialize_result() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_client_request() {
        let result = deserialize_client_request("{\"method\": \"initialize\"}");
        assert_eq!(result.method, "initialize");
    }
}
