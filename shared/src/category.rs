use serde::{Deserialize, Serialize};

/// A tag category (e.g., "artist", "character", "default").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCategoryInfo {
    pub version: Option<String>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub usages: Option<i64>,
    pub order: Option<i32>,
    pub default: Option<bool>,
}

/// A pool category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCategoryInfo {
    pub version: Option<String>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub usages: Option<i64>,
    pub default: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_tag_category_info() {
        let json = r##"{
            "version": "2024-01-15T10:30:45Z",
            "name": "character",
            "color": "#FF0000",
            "usages": 1250,
            "order": 1,
            "default": false
        }"##;
        let cat: TagCategoryInfo = serde_json::from_str(json).unwrap();
        assert_eq!(cat.name.as_deref(), Some("character"));
        assert_eq!(cat.color.as_deref(), Some("#FF0000"));
        assert_eq!(cat.usages, Some(1250));
        assert_eq!(cat.order, Some(1));
        assert_eq!(cat.default, Some(false));
    }

    #[test]
    fn deserialize_pool_category_info() {
        let json = r##"{
            "version": "2024-01-15T10:30:45Z",
            "name": "series",
            "color": "#00FF00",
            "usages": 42,
            "default": true
        }"##;
        let cat: PoolCategoryInfo = serde_json::from_str(json).unwrap();
        assert_eq!(cat.name.as_deref(), Some("series"));
        assert_eq!(cat.default, Some(true));
    }
}
