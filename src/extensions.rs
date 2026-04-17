use cloudevents::Event;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Extension attribute names
pub const EXT_KITESEQ: &str = "kiteseq";
pub const EXT_KITESUMMARY: &str = "kitesummary";
pub const EXT_KITEORIGINALHEADERS: &str = "kiteoriginalheaders";

// Federation extension attribute names
pub const EXT_KITEFEDORIGIN: &str = "kitefedorigin";
pub const EXT_KITEFEDCHAIN: &str = "kitefedchain";
pub const EXT_KITEFEDHOPS: &str = "kitefedhops";

/// Kite-specific CloudEvent extensions bundled together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiteExtensions {
    pub seq: u64,
    pub summary: String,
    pub original_headers: HashMap<String, String>,
}

/// Set the `kiteseq` extension on a CloudEvent.
pub fn set_kiteseq(event: &mut Event, seq: u64) {
    event.set_extension(EXT_KITESEQ, seq as i64);
}

/// Get the `kiteseq` extension from a CloudEvent.
pub fn get_kiteseq(event: &Event) -> Option<u64> {
    event.extension(EXT_KITESEQ).and_then(|v| match v {
        cloudevents::event::ExtensionValue::Integer(i) => Some(*i as u64),
        cloudevents::event::ExtensionValue::String(s) => s.parse().ok(),
        _ => None,
    })
}

/// Set the `kitesummary` extension on a CloudEvent.
pub fn set_kitesummary(event: &mut Event, summary: impl Into<String>) {
    event.set_extension(EXT_KITESUMMARY, summary.into());
}

/// Get the `kitesummary` extension from a CloudEvent.
pub fn get_kitesummary(event: &Event) -> Option<String> {
    event.extension(EXT_KITESUMMARY).and_then(|v| match v {
        cloudevents::event::ExtensionValue::String(s) => Some(s.clone()),
        _ => None,
    })
}

/// Set the `kiteoriginalheaders` extension on a CloudEvent.
/// Headers are serialized as a JSON string.
pub fn set_kiteoriginalheaders(event: &mut Event, headers: &HashMap<String, String>) {
    if let Ok(json) = serde_json::to_string(headers) {
        event.set_extension(EXT_KITEORIGINALHEADERS, json);
    }
}

/// Get the `kiteoriginalheaders` extension from a CloudEvent.
pub fn get_kiteoriginalheaders(event: &Event) -> Option<HashMap<String, String>> {
    event
        .extension(EXT_KITEORIGINALHEADERS)
        .and_then(|v| match v {
            cloudevents::event::ExtensionValue::String(s) => serde_json::from_str(s).ok(),
            _ => None,
        })
}

/// Set the `kitefedorigin` extension — the originating instance ID.
pub fn set_kitefedorigin(event: &mut Event, origin: impl Into<String>) {
    event.set_extension(EXT_KITEFEDORIGIN, origin.into());
}

/// Get the `kitefedorigin` extension from a CloudEvent.
pub fn get_kitefedorigin(event: &Event) -> Option<String> {
    event.extension(EXT_KITEFEDORIGIN).and_then(|v| match v {
        cloudevents::event::ExtensionValue::String(s) => Some(s.clone()),
        _ => None,
    })
}

/// Set the `kitefedchain` extension — ordered list of instance IDs traversed.
/// Serialized as a JSON array string.
pub fn set_kitefedchain(event: &mut Event, chain: &[String]) {
    if let Ok(json) = serde_json::to_string(chain) {
        event.set_extension(EXT_KITEFEDCHAIN, json);
    }
}

/// Get the `kitefedchain` extension from a CloudEvent.
/// Returns an empty vec if not present or not parseable.
pub fn get_kitefedchain(event: &Event) -> Vec<String> {
    event
        .extension(EXT_KITEFEDCHAIN)
        .and_then(|v| match v {
            cloudevents::event::ExtensionValue::String(s) => {
                serde_json::from_str::<Vec<String>>(s).ok()
            }
            _ => None,
        })
        .unwrap_or_default()
}

/// Set the `kitefedhops` extension — number of federation hops.
pub fn set_kitefedhops(event: &mut Event, hops: i64) {
    event.set_extension(EXT_KITEFEDHOPS, hops);
}

/// Get the `kitefedhops` extension from a CloudEvent.
pub fn get_kitefedhops(event: &Event) -> Option<i64> {
    event.extension(EXT_KITEFEDHOPS).and_then(|v| match v {
        cloudevents::event::ExtensionValue::Integer(i) => Some(*i),
        cloudevents::event::ExtensionValue::String(s) => s.parse().ok(),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudevents::EventBuilder;
    use cloudevents::EventBuilderV10;

    #[test]
    fn test_kiteseq_roundtrip() {
        let mut event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        set_kiteseq(&mut event, 42);
        assert_eq!(get_kiteseq(&event), Some(42));
    }

    #[test]
    fn test_kitesummary_roundtrip() {
        let mut event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        set_kitesummary(&mut event, "john pushed 3 commits to main");
        assert_eq!(
            get_kitesummary(&event),
            Some("john pushed 3 commits to main".to_string())
        );
    }

    #[test]
    fn test_kitefedorigin_roundtrip() {
        let mut event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        set_kitefedorigin(&mut event, "instance-a");
        assert_eq!(get_kitefedorigin(&event), Some("instance-a".to_string()));
    }

    #[test]
    fn test_kitefedchain_roundtrip() {
        let mut event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        let chain = vec!["instance-a".to_string(), "instance-b".to_string()];
        set_kitefedchain(&mut event, &chain);
        assert_eq!(get_kitefedchain(&event), chain);
    }

    #[test]
    fn test_kitefedchain_empty_default() {
        let event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        assert_eq!(get_kitefedchain(&event), Vec::<String>::new());
    }

    #[test]
    fn test_kitefedhops_roundtrip() {
        let mut event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        set_kitefedhops(&mut event, 3);
        assert_eq!(get_kitefedhops(&event), Some(3));
    }

    #[test]
    fn test_kiteoriginalheaders_roundtrip() {
        let mut event = EventBuilderV10::new()
            .id("test-1")
            .ty("com.github.push")
            .source("https://github.com/test/repo")
            .build()
            .unwrap();

        let mut headers = HashMap::new();
        headers.insert("X-GitHub-Event".to_string(), "push".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        set_kiteoriginalheaders(&mut event, &headers);
        let got = get_kiteoriginalheaders(&event).unwrap();
        assert_eq!(got.get("X-GitHub-Event"), Some(&"push".to_string()));
        assert_eq!(
            got.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }
}
