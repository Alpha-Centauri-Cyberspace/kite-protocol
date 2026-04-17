/// `com.kite.agent.message` event type support.
///
/// Agent messages allow AI agents to send structured messages to each other
/// via the Kite relay. Each message is a CloudEvent with type
/// `com.kite.agent.message` and a JSON payload conforming to [`AgentMessage`].
///
/// # Example CloudEvent
/// ```json
/// {
///   "specversion": "1.0",
///   "type": "com.kite.agent.message",
///   "source": "https://getkite.sh/agents/my-agent",
///   "id": "01HXYZ...",
///   "data": {
///     "from": "my-agent",
///     "to": "other-agent",
///     "body": "Here are the results you asked for.",
///     "thread_id": "thread-abc123",
///     "reply_to_id": "01HABC..."
///   }
/// }
/// ```
use serde::{Deserialize, Serialize};

/// CloudEvent type for agent-to-agent messages.
pub const EVENT_TYPE: &str = "com.kite.agent.message";

/// Source URI prefix for agent messages.
pub const SOURCE_PREFIX: &str = "https://getkite.sh/agents/";

/// Payload schema for `com.kite.agent.message` CloudEvents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Identifier of the sending agent (e.g. agent slug or ID).
    pub from: String,

    /// Identifier of the intended recipient agent.
    pub to: String,

    /// Human-readable message body.
    pub body: String,

    /// Optional thread identifier for grouping related messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,

    /// Optional CloudEvent ID this message is replying to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_id: Option<String>,
}

impl AgentMessage {
    /// Create a new agent message.
    pub fn new(from: impl Into<String>, to: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            body: body.into(),
            thread_id: None,
            reply_to_id: None,
        }
    }

    /// Attach a thread ID for conversation grouping.
    pub fn with_thread(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = Some(thread_id.into());
        self
    }

    /// Mark this as a reply to a specific CloudEvent ID.
    pub fn with_reply_to(mut self, event_id: impl Into<String>) -> Self {
        self.reply_to_id = Some(event_id.into());
        self
    }

    /// Derive the CloudEvent `source` URI for a given agent identifier.
    pub fn source_for(agent_id: &str) -> String {
        format!("{SOURCE_PREFIX}{agent_id}")
    }

    /// Generate a one-line human-readable summary for `kitesummary`.
    pub fn summary(&self) -> String {
        let body_preview = if self.body.len() > 80 {
            format!("{}…", &self.body[..80])
        } else {
            self.body.clone()
        };
        format!("[agent msg] {} → {}: {}", self.from, self.to, body_preview)
    }

    /// Parse an [`AgentMessage`] from a CloudEvent data field.
    /// Returns `None` if the event type doesn't match or data can't be parsed.
    pub fn from_event(event: &cloudevents::Event) -> Option<Self> {
        use cloudevents::AttributesReader;
        if event.ty() != EVENT_TYPE {
            return None;
        }
        match event.data() {
            Some(cloudevents::Data::Json(v)) => serde_json::from_value(v.clone()).ok(),
            Some(cloudevents::Data::String(s)) => serde_json::from_str(s).ok(),
            Some(cloudevents::Data::Binary(b)) => serde_json::from_slice(b).ok(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_message_new() {
        let msg = AgentMessage::new("agent-a", "agent-b", "hello");
        assert_eq!(msg.from, "agent-a");
        assert_eq!(msg.to, "agent-b");
        assert_eq!(msg.body, "hello");
        assert!(msg.thread_id.is_none());
        assert!(msg.reply_to_id.is_none());
    }

    #[test]
    fn agent_message_with_thread_and_reply() {
        let msg = AgentMessage::new("a", "b", "hi")
            .with_thread("t1")
            .with_reply_to("evt-001");
        assert_eq!(msg.thread_id.as_deref(), Some("t1"));
        assert_eq!(msg.reply_to_id.as_deref(), Some("evt-001"));
    }

    #[test]
    fn summary_short_body() {
        let msg = AgentMessage::new("alice", "bob", "short message");
        assert_eq!(msg.summary(), "[agent msg] alice → bob: short message");
    }

    #[test]
    fn summary_long_body_truncated() {
        let long = "a".repeat(100);
        let msg = AgentMessage::new("alice", "bob", long);
        let summary = msg.summary();
        assert!(summary.ends_with('…'));
        // preview is 80 chars + ellipsis
        assert!(summary.contains("alice → bob"));
    }

    #[test]
    fn source_for() {
        assert_eq!(
            AgentMessage::source_for("my-agent"),
            "https://getkite.sh/agents/my-agent"
        );
    }

    #[test]
    fn from_event_wrong_type_returns_none() {
        use cloudevents::EventBuilder;
        use cloudevents::EventBuilderV10;
        let event = EventBuilderV10::new()
            .id("1")
            .ty("com.github.push")
            .source("https://github.com")
            .build()
            .unwrap();
        assert!(AgentMessage::from_event(&event).is_none());
    }

    #[test]
    fn from_event_correct_type_parses() {
        use cloudevents::{EventBuilder, EventBuilderV10};
        let msg = AgentMessage::new("a", "b", "test body");
        let data = serde_json::to_value(&msg).unwrap();
        let event = EventBuilderV10::new()
            .id("1")
            .ty(EVENT_TYPE)
            .source("https://getkite.sh/agents/a")
            .data("application/json", data)
            .build()
            .unwrap();
        let parsed = AgentMessage::from_event(&event).expect("should parse");
        assert_eq!(parsed.from, "a");
        assert_eq!(parsed.to, "b");
        assert_eq!(parsed.body, "test body");
    }
}
