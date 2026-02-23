use serde::{Deserialize, Serialize};

/// A tag resource stripped down to names, category, and usage count.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroTag {
    pub names: Vec<String>,
    pub category: String,
    pub usages: i64,
}

/// Full tag resource. All fields optional to support field selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagInfo {
    pub version: Option<String>,
    pub description: Option<String>,
    pub creation_time: Option<String>,
    pub last_edit_time: Option<String>,
    pub category: Option<String>,
    pub names: Option<Vec<String>>,
    pub implications: Option<Vec<MicroTag>>,
    pub suggestions: Option<Vec<MicroTag>>,
    pub usages: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_micro_tag() {
        let json = r#"{"names": ["landscape", "scenery"], "category": "default", "usages": 42}"#;
        let tag: MicroTag = serde_json::from_str(json).unwrap();
        assert_eq!(tag.names, vec!["landscape", "scenery"]);
        assert_eq!(tag.category, "default");
        assert_eq!(tag.usages, 42);
    }
}
