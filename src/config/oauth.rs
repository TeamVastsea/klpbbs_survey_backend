use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OAuthConfig {
    #[serde_inline_default(String::from("222"))]
    pub app_id: String,
    #[serde_inline_default(String::from("111"))]
    pub app_key: String,
}

