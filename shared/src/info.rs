use serde::{Deserialize, Serialize};

use crate::enums::UserRank;
use crate::post::PostInfo;

/// Response from `GET /info`.
/// Note: This type does NOT use `rename_all = "camelCase"` — the server serializes
/// field names as snake_case for this endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoResponse {
    pub post_count: i64,
    pub disk_usage: i64,
    pub featured_post: Option<PostInfo>,
    pub featuring_time: Option<String>,
    pub featuring_user: Option<String>,
    pub server_time: String,
    pub config: PublicConfig,
}

/// Public server configuration, delivered as part of `InfoResponse`.
/// Uses camelCase for most fields, with explicit rename for `userNameRegex`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicConfig {
    pub name: String,
    pub default_user_rank: UserRank,
    pub enable_safety: bool,
    pub contact_email: Option<String>,
    #[serde(default)]
    pub can_send_mails: bool,
    /// Note: The server explicitly renames this to "userNameRegex" (capital N),
    /// overriding the camelCase default which would produce "usernameRegex".
    #[serde(rename = "userNameRegex")]
    pub username_regex: String,
    pub password_regex: String,
    pub tag_name_regex: String,
    pub tag_category_name_regex: String,
    pub privileges: PrivilegeConfig,
}

/// Privilege configuration mapping action names to required user ranks.
/// Field names are snake_case (no rename_all) — this matches the server's serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeConfig {
    pub user_create_self: UserRank,
    pub user_create_any: UserRank,
    pub user_list: UserRank,
    pub user_view: UserRank,
    pub user_edit_any_name: UserRank,
    pub user_edit_any_pass: UserRank,
    pub user_edit_any_email: UserRank,
    pub user_edit_any_avatar: UserRank,
    pub user_edit_any_rank: UserRank,
    pub user_edit_self_name: UserRank,
    pub user_edit_self_pass: UserRank,
    pub user_edit_self_email: UserRank,
    pub user_edit_self_avatar: UserRank,
    pub user_edit_self_rank: UserRank,
    pub user_delete_any: UserRank,
    pub user_delete_self: UserRank,

    pub user_token_list_any: UserRank,
    pub user_token_list_self: UserRank,
    pub user_token_create_any: UserRank,
    pub user_token_create_self: UserRank,
    pub user_token_edit_any: UserRank,
    pub user_token_edit_self: UserRank,
    pub user_token_delete_any: UserRank,
    pub user_token_delete_self: UserRank,

    pub post_create_anonymous: UserRank,
    pub post_create_identified: UserRank,
    pub post_list: UserRank,
    pub post_reverse_search: UserRank,
    pub post_view: UserRank,
    pub post_view_featured: UserRank,
    pub post_edit_content: UserRank,
    pub post_edit_description: UserRank,
    pub post_edit_flag: UserRank,
    pub post_edit_note: UserRank,
    pub post_edit_relation: UserRank,
    pub post_edit_safety: UserRank,
    pub post_edit_source: UserRank,
    pub post_edit_tag: UserRank,
    pub post_edit_thumbnail: UserRank,
    pub post_feature: UserRank,
    pub post_delete: UserRank,
    pub post_score: UserRank,
    pub post_merge: UserRank,
    pub post_favorite: UserRank,
    pub post_bulk_edit_tag: UserRank,
    pub post_bulk_edit_safety: UserRank,
    pub post_bulk_edit_delete: UserRank,

    pub tag_create: UserRank,
    pub tag_edit_name: UserRank,
    pub tag_edit_category: UserRank,
    pub tag_edit_description: UserRank,
    pub tag_edit_implication: UserRank,
    pub tag_edit_suggestion: UserRank,
    pub tag_list: UserRank,
    pub tag_view: UserRank,
    pub tag_merge: UserRank,
    pub tag_delete: UserRank,

    pub tag_category_create: UserRank,
    pub tag_category_edit_name: UserRank,
    pub tag_category_edit_color: UserRank,
    pub tag_category_edit_order: UserRank,
    pub tag_category_list: UserRank,
    pub tag_category_view: UserRank,
    pub tag_category_delete: UserRank,
    pub tag_category_set_default: UserRank,

    pub pool_create: UserRank,
    pub pool_edit_name: UserRank,
    pub pool_edit_category: UserRank,
    pub pool_edit_description: UserRank,
    pub pool_edit_post: UserRank,
    pub pool_list: UserRank,
    pub pool_view: UserRank,
    pub pool_merge: UserRank,
    pub pool_delete: UserRank,

    pub pool_category_create: UserRank,
    pub pool_category_edit_name: UserRank,
    pub pool_category_edit_color: UserRank,
    pub pool_category_list: UserRank,
    pub pool_category_view: UserRank,
    pub pool_category_delete: UserRank,
    pub pool_category_set_default: UserRank,

    pub comment_create: UserRank,
    pub comment_delete_any: UserRank,
    pub comment_delete_own: UserRank,
    pub comment_edit_any: UserRank,
    pub comment_edit_own: UserRank,
    pub comment_list: UserRank,
    pub comment_view: UserRank,
    pub comment_score: UserRank,

    pub snapshot_list: UserRank,

    pub upload_create: UserRank,
    pub upload_use_downloader: UserRank,
}

impl PrivilegeConfig {
    /// Look up the required rank for a privilege by its name (snake_case).
    pub fn get(&self, name: &str) -> Option<UserRank> {
        match name {
            "user_create_self" => Some(self.user_create_self),
            "user_create_any" => Some(self.user_create_any),
            "user_list" => Some(self.user_list),
            "user_view" => Some(self.user_view),
            "user_edit_any_name" => Some(self.user_edit_any_name),
            "user_edit_any_pass" => Some(self.user_edit_any_pass),
            "user_edit_any_email" => Some(self.user_edit_any_email),
            "user_edit_any_avatar" => Some(self.user_edit_any_avatar),
            "user_edit_any_rank" => Some(self.user_edit_any_rank),
            "user_edit_self_name" => Some(self.user_edit_self_name),
            "user_edit_self_pass" => Some(self.user_edit_self_pass),
            "user_edit_self_email" => Some(self.user_edit_self_email),
            "user_edit_self_avatar" => Some(self.user_edit_self_avatar),
            "user_edit_self_rank" => Some(self.user_edit_self_rank),
            "user_delete_any" => Some(self.user_delete_any),
            "user_delete_self" => Some(self.user_delete_self),
            "user_token_list_any" => Some(self.user_token_list_any),
            "user_token_list_self" => Some(self.user_token_list_self),
            "user_token_create_any" => Some(self.user_token_create_any),
            "user_token_create_self" => Some(self.user_token_create_self),
            "user_token_edit_any" => Some(self.user_token_edit_any),
            "user_token_edit_self" => Some(self.user_token_edit_self),
            "user_token_delete_any" => Some(self.user_token_delete_any),
            "user_token_delete_self" => Some(self.user_token_delete_self),
            "post_create_anonymous" => Some(self.post_create_anonymous),
            "post_create_identified" => Some(self.post_create_identified),
            "post_list" => Some(self.post_list),
            "post_reverse_search" => Some(self.post_reverse_search),
            "post_view" => Some(self.post_view),
            "post_view_featured" => Some(self.post_view_featured),
            "post_edit_content" => Some(self.post_edit_content),
            "post_edit_description" => Some(self.post_edit_description),
            "post_edit_flag" => Some(self.post_edit_flag),
            "post_edit_note" => Some(self.post_edit_note),
            "post_edit_relation" => Some(self.post_edit_relation),
            "post_edit_safety" => Some(self.post_edit_safety),
            "post_edit_source" => Some(self.post_edit_source),
            "post_edit_tag" => Some(self.post_edit_tag),
            "post_edit_thumbnail" => Some(self.post_edit_thumbnail),
            "post_feature" => Some(self.post_feature),
            "post_delete" => Some(self.post_delete),
            "post_score" => Some(self.post_score),
            "post_merge" => Some(self.post_merge),
            "post_favorite" => Some(self.post_favorite),
            "post_bulk_edit_tag" => Some(self.post_bulk_edit_tag),
            "post_bulk_edit_safety" => Some(self.post_bulk_edit_safety),
            "post_bulk_edit_delete" => Some(self.post_bulk_edit_delete),
            "tag_create" => Some(self.tag_create),
            "tag_edit_name" => Some(self.tag_edit_name),
            "tag_edit_category" => Some(self.tag_edit_category),
            "tag_edit_description" => Some(self.tag_edit_description),
            "tag_edit_implication" => Some(self.tag_edit_implication),
            "tag_edit_suggestion" => Some(self.tag_edit_suggestion),
            "tag_list" => Some(self.tag_list),
            "tag_view" => Some(self.tag_view),
            "tag_merge" => Some(self.tag_merge),
            "tag_delete" => Some(self.tag_delete),
            "tag_category_create" => Some(self.tag_category_create),
            "tag_category_edit_name" => Some(self.tag_category_edit_name),
            "tag_category_edit_color" => Some(self.tag_category_edit_color),
            "tag_category_edit_order" => Some(self.tag_category_edit_order),
            "tag_category_list" => Some(self.tag_category_list),
            "tag_category_view" => Some(self.tag_category_view),
            "tag_category_delete" => Some(self.tag_category_delete),
            "tag_category_set_default" => Some(self.tag_category_set_default),
            "pool_create" => Some(self.pool_create),
            "pool_edit_name" => Some(self.pool_edit_name),
            "pool_edit_category" => Some(self.pool_edit_category),
            "pool_edit_description" => Some(self.pool_edit_description),
            "pool_edit_post" => Some(self.pool_edit_post),
            "pool_list" => Some(self.pool_list),
            "pool_view" => Some(self.pool_view),
            "pool_merge" => Some(self.pool_merge),
            "pool_delete" => Some(self.pool_delete),
            "pool_category_create" => Some(self.pool_category_create),
            "pool_category_edit_name" => Some(self.pool_category_edit_name),
            "pool_category_edit_color" => Some(self.pool_category_edit_color),
            "pool_category_list" => Some(self.pool_category_list),
            "pool_category_view" => Some(self.pool_category_view),
            "pool_category_delete" => Some(self.pool_category_delete),
            "pool_category_set_default" => Some(self.pool_category_set_default),
            "comment_create" => Some(self.comment_create),
            "comment_delete_any" => Some(self.comment_delete_any),
            "comment_delete_own" => Some(self.comment_delete_own),
            "comment_edit_any" => Some(self.comment_edit_any),
            "comment_edit_own" => Some(self.comment_edit_own),
            "comment_list" => Some(self.comment_list),
            "comment_view" => Some(self.comment_view),
            "comment_score" => Some(self.comment_score),
            "snapshot_list" => Some(self.snapshot_list),
            "upload_create" => Some(self.upload_create),
            "upload_use_downloader" => Some(self.upload_use_downloader),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_privileges_json() -> &'static str {
        r#"{
            "user_create_self": "anonymous",
            "user_create_any": "administrator",
            "user_list": "regular",
            "user_view": "regular",
            "user_edit_any_name": "moderator",
            "user_edit_any_pass": "moderator",
            "user_edit_any_email": "moderator",
            "user_edit_any_avatar": "moderator",
            "user_edit_any_rank": "moderator",
            "user_edit_self_name": "regular",
            "user_edit_self_pass": "regular",
            "user_edit_self_email": "regular",
            "user_edit_self_avatar": "regular",
            "user_edit_self_rank": "moderator",
            "user_delete_any": "administrator",
            "user_delete_self": "regular",
            "user_token_list_any": "administrator",
            "user_token_list_self": "regular",
            "user_token_create_any": "administrator",
            "user_token_create_self": "regular",
            "user_token_edit_any": "administrator",
            "user_token_edit_self": "regular",
            "user_token_delete_any": "administrator",
            "user_token_delete_self": "regular",
            "post_create_anonymous": "regular",
            "post_create_identified": "regular",
            "post_list": "anonymous",
            "post_reverse_search": "regular",
            "post_view": "anonymous",
            "post_view_featured": "anonymous",
            "post_edit_content": "regular",
            "post_edit_description": "regular",
            "post_edit_flag": "regular",
            "post_edit_note": "regular",
            "post_edit_relation": "regular",
            "post_edit_safety": "regular",
            "post_edit_source": "regular",
            "post_edit_tag": "regular",
            "post_edit_thumbnail": "regular",
            "post_feature": "moderator",
            "post_delete": "moderator",
            "post_score": "regular",
            "post_merge": "moderator",
            "post_favorite": "regular",
            "post_bulk_edit_tag": "power",
            "post_bulk_edit_safety": "power",
            "post_bulk_edit_delete": "moderator",
            "tag_create": "regular",
            "tag_edit_name": "power",
            "tag_edit_category": "power",
            "tag_edit_description": "power",
            "tag_edit_implication": "power",
            "tag_edit_suggestion": "power",
            "tag_list": "anonymous",
            "tag_view": "anonymous",
            "tag_merge": "moderator",
            "tag_delete": "moderator",
            "tag_category_create": "moderator",
            "tag_category_edit_name": "moderator",
            "tag_category_edit_color": "moderator",
            "tag_category_edit_order": "moderator",
            "tag_category_list": "anonymous",
            "tag_category_view": "anonymous",
            "tag_category_delete": "moderator",
            "tag_category_set_default": "moderator",
            "pool_create": "regular",
            "pool_edit_name": "power",
            "pool_edit_category": "power",
            "pool_edit_description": "power",
            "pool_edit_post": "power",
            "pool_list": "anonymous",
            "pool_view": "anonymous",
            "pool_merge": "moderator",
            "pool_delete": "moderator",
            "pool_category_create": "moderator",
            "pool_category_edit_name": "moderator",
            "pool_category_edit_color": "moderator",
            "pool_category_list": "anonymous",
            "pool_category_view": "anonymous",
            "pool_category_delete": "moderator",
            "pool_category_set_default": "moderator",
            "comment_create": "regular",
            "comment_delete_any": "moderator",
            "comment_delete_own": "regular",
            "comment_edit_any": "moderator",
            "comment_edit_own": "regular",
            "comment_list": "anonymous",
            "comment_view": "anonymous",
            "comment_score": "regular",
            "snapshot_list": "power",
            "upload_create": "regular",
            "upload_use_downloader": "power"
        }"#
    }

    #[test]
    fn deserialize_privilege_config() {
        let privs: PrivilegeConfig = serde_json::from_str(sample_privileges_json()).unwrap();
        assert_eq!(privs.post_list, UserRank::Anonymous);
        assert_eq!(privs.post_feature, UserRank::Moderator);
        assert_eq!(privs.user_create_any, UserRank::Administrator);
        assert_eq!(privs.post_bulk_edit_tag, UserRank::Power);
    }

    #[test]
    fn privilege_config_get_lookup() {
        let privs: PrivilegeConfig = serde_json::from_str(sample_privileges_json()).unwrap();
        assert_eq!(privs.get("post_list"), Some(UserRank::Anonymous));
        assert_eq!(privs.get("post_feature"), Some(UserRank::Moderator));
        assert_eq!(privs.get("nonexistent"), None);
    }

    #[test]
    fn deserialize_info_response() {
        let json = format!(r#"{{
            "post_count": 12345,
            "disk_usage": 1073741824,
            "featured_post": null,
            "featuring_time": null,
            "featuring_user": null,
            "server_time": "2024-01-15T10:30:45Z",
            "config": {{
                "name": "Test Booru",
                "defaultUserRank": "regular",
                "enableSafety": true,
                "contactEmail": "admin@example.com",
                "canSendMails": false,
                "userNameRegex": "^[a-zA-Z0-9_-]{{1,32}}$",
                "passwordRegex": "^.{{5,}}$",
                "tagNameRegex": "^\\S+$",
                "tagCategoryNameRegex": "^[^\\s%+#/]+$",
                "privileges": {privs}
            }}
        }}"#, privs = sample_privileges_json());
        let info: InfoResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(info.post_count, 12345);
        assert_eq!(info.disk_usage, 1073741824);
        assert!(info.featured_post.is_none());
        assert_eq!(info.config.name, "Test Booru");
        assert_eq!(info.config.default_user_rank, UserRank::Regular);
        assert!(info.config.enable_safety);
        assert_eq!(info.config.username_regex, "^[a-zA-Z0-9_-]{1,32}$");
    }

    #[test]
    fn public_config_username_regex_rename() {
        // Verify the field is serialized as "userNameRegex" (capital N), not "usernameRegex"
        let json = format!(r#"{{
            "name": "Test",
            "defaultUserRank": "anonymous",
            "enableSafety": false,
            "userNameRegex": ".*",
            "passwordRegex": ".*",
            "tagNameRegex": ".*",
            "tagCategoryNameRegex": ".*",
            "privileges": {privs}
        }}"#, privs = sample_privileges_json());
        let config: PublicConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.username_regex, ".*");

        // Roundtrip: serialized output should use "userNameRegex"
        let serialized = serde_json::to_string(&config).unwrap();
        assert!(serialized.contains("\"userNameRegex\""));
        assert!(!serialized.contains("\"usernameRegex\""));
    }
}
