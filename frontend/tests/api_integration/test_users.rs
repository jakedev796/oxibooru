use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::user::UserInfo;

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /users should return a paged response.
#[tokio::test]
async fn get_users_returns_paged_response() {
    let url = format!("{BACKEND_URL}/users?query=&offset=0&limit=5");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /users returned {}", resp.status());

    let page: PagedResponse<UserInfo> = resp
        .json()
        .await
        .expect("failed to deserialize PagedResponse<UserInfo>");
    assert!(page.total >= 0);
    assert!(page.limit == 5);
    assert!(page.results.len() <= 5);
}

/// GET /user/{name} should return a single user if any exist.
#[tokio::test]
async fn get_single_user() {
    let list_url = format!("{BACKEND_URL}/users?query=&offset=0&limit=1");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<UserInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no users in database");
        return;
    }

    let username = page.results[0].name.as_ref().unwrap().clone();
    let url = format!("{BACKEND_URL}/user/{username}");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success(), "GET /user/{username} returned {}", resp.status());

    let user: UserInfo = resp.json().await.expect("failed to deserialize UserInfo");
    assert_eq!(user.name.as_deref(), Some(username.as_str()));
}

/// GET /users with sort should work.
#[tokio::test]
async fn get_users_sorted_by_name() {
    let url = format!("{BACKEND_URL}/users?query=sort:name&offset=0&limit=3");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success());

    let page: PagedResponse<UserInfo> = resp.json().await.unwrap();
    for user in &page.results {
        assert!(user.name.is_some(), "user should have a name field");
    }
}
