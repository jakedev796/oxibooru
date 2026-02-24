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

/// A sibling tag with its co-occurrence count.
/// Returned by `GET /tag-siblings/{name}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSibling {
    pub tag: TagInfo,
    pub occurrences: i64,
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

    #[test]
    fn deserialize_tag_info_full() {
        let json = r#"{
            "version": "2024-01-15T10:30:45Z",
            "description": "Tags for landscape photos",
            "creationTime": "2024-01-01T00:00:00Z",
            "lastEditTime": "2024-01-15T10:30:45Z",
            "category": "default",
            "names": ["landscape", "scenery"],
            "implications": [{"names": ["nature"], "category": "default", "usages": 100}],
            "suggestions": [{"names": ["mountains"], "category": "default", "usages": 50}],
            "usages": 42
        }"#;
        let tag: TagInfo = serde_json::from_str(json).unwrap();
        assert_eq!(tag.names.as_ref().unwrap().len(), 2);
        assert_eq!(tag.category.as_deref(), Some("default"));
        assert_eq!(tag.usages, Some(42));
        assert_eq!(tag.implications.as_ref().unwrap().len(), 1);
        assert_eq!(tag.suggestions.as_ref().unwrap().len(), 1);
        assert_eq!(tag.description.as_deref(), Some("Tags for landscape photos"));
    }

    #[test]
    fn deserialize_tag_sibling() {
        let json = r#"{
            "tag": {
                "names": ["mountains"],
                "category": "default",
                "usages": 50
            },
            "occurrences": 25
        }"#;
        let sibling: TagSibling = serde_json::from_str(json).unwrap();
        assert_eq!(sibling.occurrences, 25);
        assert_eq!(sibling.tag.names.as_ref().unwrap(), &vec!["mountains".to_string()]);
    }
}
