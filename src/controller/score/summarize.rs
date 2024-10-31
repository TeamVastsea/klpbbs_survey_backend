use serde::Deserialize;
use crate::service::token::AdminTokenInfo;

pub async fn summarize(AdminTokenInfo(admin): AdminTokenInfo) {
    
}

#[derive(Deserialize)]
pub struct SummarizeRequest {
    pub survey: i32,
    pub only_confirmed: bool,
    pub use_score: bool,
}