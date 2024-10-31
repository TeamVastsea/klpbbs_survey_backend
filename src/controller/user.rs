use crate::controller::error::ErrorMessage;
use crate::dao::entity::prelude::User;
use crate::dao::entity::user;
use crate::dao::entity::user::UserType;
use crate::dao::model::user_data::UserData;
use crate::service::password::{generate_password_hash, verify_password};
use crate::service::token::TokenInfo;
use crate::DATABASE;
use axum::extract::{Path, Query};
use axum::{debug_handler, Json};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet};
use serde::Deserialize;

pub async fn register(Query(request): Query<RegisterLoginRequest>) {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789-";
    const SALT_LEN: usize = 16;

    let mut rng = StdRng::from_entropy();

    let id: String = (0..SALT_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let user = user::ActiveModel {
        id: Set(id),
        credential: NotSet,
        admin: NotSet,
        disabled: NotSet,
        username: Set(request.username),
        password: Set(Some(generate_password_hash(&request.password))),
        user_source: Set(UserType::Password),
    };
    user.insert(&*DATABASE).await.unwrap();
}

pub async fn login(Query(request): Query<RegisterLoginRequest>) -> Result<String, ErrorMessage> {
    let user = User::find()
        .filter(user::Column::Username.eq(&request.username))
        .one(&*DATABASE)
        .await
        .unwrap()
        .ok_or(ErrorMessage::NotFound)?;
    let Some(password_hash) = user.password.clone() else { return Err(ErrorMessage::PermissionDenied) };

    if !verify_password(&request.password, &password_hash) {
        return Err(ErrorMessage::PermissionDenied);
    }

    let user: UserData = user.into();

    Ok(user.get_token().await)
}

pub async fn change_password(TokenInfo(user): TokenInfo, Json(password): Json<ChangePasswordRequest>) -> Result<(), ErrorMessage> {
    let user = User::find()
        .filter(user::Column::Id.eq(&user.uid))
        .one(&*DATABASE)
        .await
        .unwrap()
        .ok_or_else(|| ErrorMessage::NotFound)?;
    let Some(password_hash) = user.password.clone() else { return Err(ErrorMessage::PermissionDenied) };

    if !verify_password(&password.old, &password_hash) {
        return Err(ErrorMessage::PermissionDenied);
    }

    let user: UserData = user.into();
    user.update_password(Some(&password.new)).await?;
    user.remove_token().await;

    Ok(())
}

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

#[derive(Deserialize)]
pub struct RegisterLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old: String,
    pub new: String,
}