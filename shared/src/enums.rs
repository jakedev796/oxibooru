use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// User permission level. Ordered — each rank has all privileges of ranks below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRank {
    Anonymous,
    Restricted,
    Regular,
    Power,
    Moderator,
    Administrator,
}

/// Content safety rating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostSafety {
    Safe,
    Sketchy,
    Unsafe,
}

/// The type of content a post contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Image,
    Animation,
    Video,
    Flash,
}

/// MIME type of post content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MimeType {
    #[serde(rename = "image/bmp")]
    Bmp,
    #[serde(rename = "image/gif")]
    Gif,
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/webp")]
    Webp,
    #[serde(rename = "video/mp4")]
    Mp4,
    #[serde(rename = "video/quicktime")]
    Mov,
    #[serde(rename = "video/webm")]
    Webm,
    #[serde(rename = "application/x-shockwave-flash")]
    Swf,
    #[serde(rename = "image/avif")]
    Avif,
}

/// How a user's avatar is determined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AvatarStyle {
    Gravatar,
    Manual,
}

/// A score rating: -1 (dislike), 0 (none), or 1 (like).
/// Serialized as an integer, not a string.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum Rating {
    Dislike = -1,
    #[default]
    None = 0,
    Like = 1,
}

/// The type of operation recorded in a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOperation {
    Created,
    Modified,
    Merged,
    Deleted,
}

/// The type of resource recorded in a snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Comment,
    Pool,
    PoolCategory,
    Post,
    Tag,
    TagCategory,
    TagImplication,
    TagSuggestion,
    User,
    UserToken,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_rank_ordering() {
        assert!(UserRank::Anonymous < UserRank::Regular);
        assert!(UserRank::Regular < UserRank::Power);
        assert!(UserRank::Power < UserRank::Moderator);
        assert!(UserRank::Moderator < UserRank::Administrator);
    }

    #[test]
    fn user_rank_serde_roundtrip() {
        let rank = UserRank::Administrator;
        let json = serde_json::to_string(&rank).unwrap();
        assert_eq!(json, r#""administrator""#);
        let parsed: UserRank = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, rank);
    }

    #[test]
    fn post_safety_serde_roundtrip() {
        let safety = PostSafety::Sketchy;
        let json = serde_json::to_string(&safety).unwrap();
        assert_eq!(json, r#""sketchy""#);
        let parsed: PostSafety = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, safety);
    }

    #[test]
    fn mime_type_serde_roundtrip() {
        let mime = MimeType::Jpeg;
        let json = serde_json::to_string(&mime).unwrap();
        assert_eq!(json, r#""image/jpeg""#);
        let parsed: MimeType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, mime);
    }

    #[test]
    fn rating_serde_as_integer() {
        let like = Rating::Like;
        let json = serde_json::to_string(&like).unwrap();
        assert_eq!(json, "1");
        let parsed: Rating = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, like);

        let dislike = Rating::Dislike;
        let json = serde_json::to_string(&dislike).unwrap();
        assert_eq!(json, "-1");

        let none = Rating::None;
        let json = serde_json::to_string(&none).unwrap();
        assert_eq!(json, "0");
    }

    #[test]
    fn resource_operation_serde() {
        let op = ResourceOperation::Modified;
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, r#""Modified""#);
        let parsed: ResourceOperation = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, op);
    }
}
