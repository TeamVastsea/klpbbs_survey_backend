use crate::DATABASE;
use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::User;
use crate::dao::entity::user;
use crate::dao::model::PagedData;
use crate::dao::model::user_data::UserData;
use crate::service::token::{AdminTokenInfo, TokenInfo, validate_token_time};
use axum::Json;
use axum::extract::{Path, Query};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use tracing::info;

pub async fn get_user_info(TokenInfo(user): TokenInfo) -> String {
    serde_json::to_string(&user).unwrap()
}

pub async fn get_other_user_info(
    TokenInfo(user): TokenInfo,
    Path(other): Path<String>,
) -> Result<String, ErrorMessage> {
    let Some(other) = UserData::find_by_id(&other).await else {
        return Err(ErrorMessage::NotFound);
    };

    if !other.admin && !user.admin {
        return Err(ErrorMessage::PermissionDenied);
    }

    Ok(serde_json::to_string(&other).unwrap())
}

pub async fn invalidate_token(TokenInfo(user): TokenInfo) -> Result<(), ErrorMessage> {
    user.remove_token().await;
    Ok(())
}

pub async fn list_users(
    AdminTokenInfo(admin): AdminTokenInfo,
    Query(query): Query<ListUsersQuery>,
) -> Result<String, ErrorMessage> {
    info!("Admin {} list users", admin.uid);

    let size = min(max(query.size.unwrap_or(20), 1), 100);
    let mut users = User::find().order_by_asc(user::Column::Id);

    if let Some(search) = query
        .search
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        let pattern = format!("%{}%", search);
        users = users.filter(
            Condition::any()
                .add(user::Column::Id.like(&pattern))
                .add(user::Column::Username.like(&pattern)),
        );
    }

    let users = users.paginate(&*DATABASE, size);
    let data = users
        .fetch_page(query.page.unwrap_or(0))
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .into_iter()
        .map(ManagedUser::from)
        .collect();
    let total = users
        .num_pages()
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;

    serde_json::to_string(&PagedData { data, total })
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))
}

pub async fn update_user(
    AdminTokenInfo(admin): AdminTokenInfo,
    Path(uid): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<String, ErrorMessage> {
    ensure_other_user(&admin, &uid)?;
    if request.admin.is_none() && request.disabled.is_none() {
        return Err(ErrorMessage::InvalidParams("admin or disabled".to_string()));
    }

    let user = User::find_by_id(&uid)
        .one(&*DATABASE)
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;
    let mut user = user.into_active_model();

    if let Some(value) = request.admin {
        user.admin = Set(value);
    }
    if let Some(value) = request.disabled {
        user.disabled = Set(value);
        if value {
            user.credential = Set(None);
        }
    }

    let user = user
        .update(&*DATABASE)
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    info!("Admin {} updated user {}", admin.uid, uid);

    serde_json::to_string(&ManagedUser::from(user))
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))
}

pub async fn invalidate_user_sessions(
    AdminTokenInfo(admin): AdminTokenInfo,
    Path(uid): Path<String>,
) -> Result<String, ErrorMessage> {
    ensure_other_user(&admin, &uid)?;

    let user = User::find_by_id(&uid)
        .one(&*DATABASE)
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?
        .ok_or(ErrorMessage::NotFound)?;
    let mut user = user.into_active_model();
    user.credential = Set(None);
    user.update(&*DATABASE)
        .await
        .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))?;
    info!("Admin {} invalidated sessions for user {}", admin.uid, uid);

    Ok(uid)
}

fn ensure_other_user(admin: &UserData, uid: &str) -> Result<(), ErrorMessage> {
    if admin.uid == uid {
        Err(ErrorMessage::InvalidParams(
            "cannot manage current user".to_string(),
        ))
    } else {
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct ListUsersQuery {
    page: Option<u64>,
    size: Option<u64>,
    search: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    admin: Option<bool>,
    disabled: Option<bool>,
}

#[derive(Serialize)]
pub struct ManagedUser {
    uid: String,
    username: String,
    admin: bool,
    disabled: bool,
    logged_in: bool,
}

impl From<user::Model> for ManagedUser {
    fn from(user: user::Model) -> Self {
        Self {
            uid: user.id,
            username: user.username,
            admin: user.admin,
            disabled: user.disabled,
            logged_in: user.credential.as_deref().is_some_and(validate_token_time),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ensure_other_user;
    use crate::dao::model::user_data::UserData;

    #[test]
    fn administrator_cannot_manage_self() {
        let admin = UserData {
            uid: "admin".to_string(),
            username: "Admin".to_string(),
            admin: true,
        };

        assert!(ensure_other_user(&admin, "admin").is_err());
        assert!(ensure_other_user(&admin, "other").is_ok());
    }
}
