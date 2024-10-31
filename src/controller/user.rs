use crate::controller::error::ErrorMessage;
use crate::dao::model::user_data::UserData;
use crate::service::token::TokenInfo;
use axum::extract::Path;


pub async fn get_user_info(TokenInfo(user): TokenInfo) -> String {
    serde_json::to_string(&user).unwrap()
}

pub async fn get_other_user_info(TokenInfo(user): TokenInfo, Path(other): Path<String>) -> Result<String, ErrorMessage> {
    let Some(other) = UserData::find_by_id(&other).await else { return Err(ErrorMessage::NotFound) };

    if !other.admin && !user.admin {
        return Err(ErrorMessage::PermissionDenied);
    }

    Ok(serde_json::to_string(&other).unwrap())
}

pub async fn invalidate_token(TokenInfo(user): TokenInfo) -> Result<(), ErrorMessage> {
    user.remove_token().await;
    Ok(())
}