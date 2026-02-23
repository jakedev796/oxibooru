use serde::{Deserialize, Serialize};

use crate::enums::{ResourceOperation, ResourceType};
use crate::user::MicroUser;

/// A record of a change made to a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub user: Option<Option<MicroUser>>,
    pub operation: Option<ResourceOperation>,
    #[serde(rename = "type")]
    pub resource_type: Option<ResourceType>,
    #[serde(rename = "id")]
    pub resource_id: Option<String>,
    pub data: Option<serde_json::Value>,
    pub time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{ResourceOperation, ResourceType};

    #[test]
    fn deserialize_snapshot_info() {
        let json = r#"{
            "user": {"name": "admin", "avatarUrl": "/avatars/admin.jpg"},
            "operation": "Modified",
            "type": "Tag",
            "id": "character",
            "data": {"type": "object change", "value": {}},
            "time": "2024-01-15T10:30:45Z"
        }"#;
        let snap: SnapshotInfo = serde_json::from_str(json).unwrap();
        assert_eq!(snap.operation, Some(ResourceOperation::Modified));
        assert_eq!(snap.resource_type, Some(ResourceType::Tag));
        assert_eq!(snap.resource_id.as_deref(), Some("character"));
        assert!(snap.data.is_some());
        assert_eq!(snap.user.unwrap().unwrap().name, "admin");
    }

    #[test]
    fn deserialize_snapshot_with_null_user() {
        let json = r#"{
            "user": null,
            "operation": "Created",
            "type": "Post",
            "id": "123",
            "time": "2024-01-15T10:30:45Z"
        }"#;
        let snap: SnapshotInfo = serde_json::from_str(json).unwrap();
        assert!(snap.user.is_none());
        assert_eq!(snap.resource_type, Some(ResourceType::Post));
    }
}
