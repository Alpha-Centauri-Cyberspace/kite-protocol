use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Messages sent from the client to the server over WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "bindings/")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Initial handshake after WebSocket connection is established.
    Connect {
        version: u32,
        token: String,
        /// Team context for this connection.
        team_id: String,
        /// Source/type subscriptions, e.g. ["source:github", "type:push"] or ["*"]
        scopes: Vec<String>,
        /// Optional stable client identifier for cursor resumption.
        #[serde(default)]
        client_id: Option<String>,
    },
    /// Acknowledge receipt of events up to this sequence number.
    Ack { seq: u64 },
    /// RPC request from client.
    Request {
        id: String,
        method: String,
        #[serde(default)]
        params: serde_json::Value,
    },
}

/// Messages sent from the server to the client over WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "bindings/")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Handshake acknowledgment.
    Connected { last_seq: u64, client_id: String },
    /// Sent when the client's cursor points beyond retained events.
    CursorExpired {
        oldest_available_seq: u64,
        message: String,
    },
    /// A CloudEvent pushed from the server.
    Event {
        seq: u64,
        /// The CloudEvent serialized as JSON value.
        /// We use serde_json::Value here because cloudevents::Event
        /// doesn't implement TS, and for WS transport we serialize it anyway.
        event: serde_json::Value,
    },
    /// Response to a client RPC request.
    Response {
        id: String,
        ok: bool,
        #[serde(default)]
        payload: serde_json::Value,
    },
    /// Error frame.
    Error { code: u32, message: String },
    /// Quota snapshot sent after connection is established.
    QuotaSnapshot {
        provider: String,
        events_used: i64,
        events_limit: i64,
        current_period_end: String,
    },
    /// Billing block: team is over quota or has insufficient balance.
    BillingBlock {
        provider: String,
        reason: String,
        retryable: bool,
        action: serde_json::Value,
    },
}

// Error codes
pub const ERR_AUTH_FAILED: u32 = 4001;
pub const ERR_INVALID_MESSAGE: u32 = 4002;
pub const ERR_RATE_LIMITED: u32 = 4003;
pub const ERR_SLOW_CONSUMER: u32 = 4004;
pub const ERR_QUOTA_BLOCKED: u32 = 4005;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_connect_serialization() {
        let msg = ClientMessage::Connect {
            version: 1,
            token: "test-token-fixture".to_string(),
            team_id: "team_123".to_string(),
            scopes: vec!["source:github".to_string(), "type:push".to_string()],
            client_id: Some("my-agent".to_string()),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"connect\""));
        assert!(json.contains("\"version\":1"));
        assert!(json.contains("\"client_id\":\"my-agent\""));

        let deserialized: ClientMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            ClientMessage::Connect {
                version,
                team_id,
                scopes,
                client_id,
                ..
            } => {
                assert_eq!(version, 1);
                assert_eq!(team_id, "team_123");
                assert_eq!(scopes.len(), 2);
                assert_eq!(client_id, Some("my-agent".to_string()));
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_client_message_ack_serialization() {
        let msg = ClientMessage::Ack { seq: 42 };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"ack\""));
        assert!(json.contains("\"seq\":42"));

        let deserialized: ClientMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            ClientMessage::Ack { seq } => assert_eq!(seq, 42),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_connect_without_client_id() {
        let json = r#"{"type":"connect","version":1,"token":"t","team_id":"t","scopes":["*"]}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Connect { client_id, .. } => assert_eq!(client_id, None),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_server_message_event_serialization() {
        let msg = ServerMessage::Event {
            seq: 42,
            event: serde_json::json!({
                "specversion": "1.0",
                "id": "test-1",
                "type": "com.github.push",
                "source": "https://github.com/test/repo"
            }),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"event\""));
        assert!(json.contains("\"seq\":42"));

        let deserialized: ServerMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            ServerMessage::Event { seq, .. } => assert_eq!(seq, 42),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_server_message_error_serialization() {
        let msg = ServerMessage::Error {
            code: ERR_AUTH_FAILED,
            message: "Invalid API key".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"code\":4001"));
    }
}
