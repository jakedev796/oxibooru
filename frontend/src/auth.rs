use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use oxibooru_shared::enums::UserRank;
use oxibooru_shared::info::{InfoResponse, PrivilegeConfig};
use oxibooru_shared::user::UserInfo;

use crate::api::{ApiClient, ApiError, Credentials};

const STORAGE_KEY_USERNAME: &str = "oxibooru-auth-username";
const STORAGE_KEY_PASSWORD: &str = "oxibooru-auth-password";

/// Auth state provided as Leptos context.
#[derive(Copy, Clone)]
pub struct AuthState {
    pub current_user: RwSignal<Option<UserInfo>>,
    pub privileges: RwSignal<Option<PrivilegeConfig>>,
    api: RwSignal<ApiClient>,
}

impl AuthState {
    pub fn new(api: RwSignal<ApiClient>) -> Self {
        let state = Self {
            current_user: RwSignal::new(None),
            privileges: RwSignal::new(None),
            api,
        };

        // Restore credentials from localStorage
        if let (Ok(username), Ok(password)) =
            (LocalStorage::get::<String>(STORAGE_KEY_USERNAME), LocalStorage::get::<String>(STORAGE_KEY_PASSWORD))
        {
            if !username.is_empty() && !password.is_empty() {
                api.update(|client| {
                    client.set_credentials(Some(Credentials::Basic { username, password }));
                });
            }
        }

        state
    }

    /// Log in with username and password.
    /// Verifies credentials via `/info?bump-login=true`, then fetches the user profile.
    pub async fn login(&self, username: String, password: String) -> Result<InfoResponse, ApiError> {
        // Set credentials on the API client
        let creds = Credentials::Basic {
            username: username.clone(),
            password: password.clone(),
        };
        self.api.update(|client| {
            client.set_credentials(Some(creds));
        });

        // Verify credentials by fetching /info with bump-login
        let info = self.api.get_untracked().get_info_bump_login().await;

        match &info {
            Ok(resp) => {
                // Store credentials
                let _ = LocalStorage::set(STORAGE_KEY_USERNAME, &username);
                let _ = LocalStorage::set(STORAGE_KEY_PASSWORD, &password);
                self.privileges.set(Some(resp.config.privileges.clone()));

                // Fetch and set current user profile
                if let Ok(user) = self.api.get_untracked().get_user(&username).await {
                    self.current_user.set(Some(user));
                }
            }
            Err(_) => {
                // Clear credentials on failure
                self.api.update(|client| {
                    client.set_credentials(None);
                });
            }
        }

        info
    }

    /// Verify stored credentials and populate current_user on app startup.
    /// Should be called once after AuthState::new() if credentials were restored.
    pub async fn verify_session(&self) {
        // Only try if credentials are set
        let has_creds = self.api.get_untracked().has_credentials();
        if !has_creds {
            return;
        }

        match self.api.get_untracked().get_info_bump_login().await {
            Ok(resp) => {
                self.privileges.set(Some(resp.config.privileges.clone()));
                // Extract username from stored credentials
                if let Ok(username) = LocalStorage::get::<String>(STORAGE_KEY_USERNAME) {
                    if let Ok(user) = self.api.get_untracked().get_user(&username).await {
                        self.current_user.set(Some(user));
                    }
                }
            }
            Err(_) => {
                // Stored credentials are invalid — clear them
                self.logout();
            }
        }
    }

    /// Log out — clear credentials and user state.
    pub fn logout(&self) {
        self.current_user.set(None);
        self.api.update(|client| {
            client.set_credentials(None);
        });
        LocalStorage::delete(STORAGE_KEY_USERNAME);
        LocalStorage::delete(STORAGE_KEY_PASSWORD);
    }

    /// Check if the current user is logged in.
    pub fn is_logged_in(&self) -> bool {
        self.current_user.get_untracked().is_some()
    }

    /// Check if the current user has a given privilege.
    pub fn has_privilege(&self, name: &str) -> bool {
        let Some(privs) = self.privileges.get_untracked() else {
            return false;
        };
        let Some(required_rank) = privs.get(name) else {
            return false;
        };
        let user_rank = self
            .current_user
            .get_untracked()
            .and_then(|u| u.rank)
            .unwrap_or(UserRank::Anonymous);
        user_rank >= required_rank
    }
}
