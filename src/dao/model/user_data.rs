use crate::dao::entity::prelude::User;
use crate::dao::entity::user;
use crate::DATABASE;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, NotSet, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use crate::controller::error::ErrorMessage;
use crate::dao::entity::user::UserType;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub uid: String,
    pub username: String,
    pub admin: bool,
    pub source: UserType,
}

impl UserData {
    pub async fn find_by_id(uid: &str) -> Option<Self> {
        User::find_by_id(uid)
            .filter(user::Column::Disabled.eq(false))
            .one(&*DATABASE)
            .await
            .ok()
            .flatten()
            .map(|user| user.into())
    }

    pub async fn get_by_credential(credential: &str) -> Option<Self> {
        User::find()
            .filter(user::Column::Credential.eq(credential))
            .filter(user::Column::Disabled.eq(false))
            .one(&*DATABASE)
            .await
            .ok()
            .flatten()
            .map(|user| user.into())
    }

    pub async fn save(&self) -> Result<(), sea_orm::error::DbErr> {
        let user = user::ActiveModel {
            id: Set(self.uid.clone()),
            credential: NotSet,
            admin: Set(self.admin),
            disabled: NotSet,
            username: Set(self.username.clone()),
            password: NotSet,
            user_source: Set(self.source),
        };

        user.insert(&*DATABASE).await.map(|_| ())
    }

    pub async fn get_credentials(&self) -> Option<String> {
        #[derive(FromQueryResult)]
        struct UserCredential {
            credential: Option<String>,
        }

        let user = User::find()
            .filter(user::Column::Id.eq(&self.uid))
            .select_only()
            .column(user::Column::Credential)
            .into_model::<UserCredential>()
            .one(&*DATABASE)
            .await
            .ok()
            .flatten();

        user.and_then(|u| u.credential)
    }

    pub async fn update_credentials(&self, credential: Option<&str>) -> Result<(), sea_orm::error::DbErr> {
        let user = user::ActiveModel {
            id: Set(self.uid.clone()),
            credential: Set(credential.map(|s| s.to_string())),
            admin: NotSet,
            disabled: NotSet,
            username: NotSet,
            password: NotSet,
            user_source: NotSet,
        };

        user.update(&*DATABASE).await.map(|_| ())
    }
    
    pub async fn update_password(&self, password: Option<&str>) -> Result<(), ErrorMessage> {
        let user = user::ActiveModel {
            id: Set(self.uid.clone()),
            credential: NotSet,
            admin: NotSet,
            disabled: NotSet,
            username: NotSet,
            password: Set(password.map(|s| s.to_string())),
            user_source: NotSet,
        };

        user.update(&*DATABASE).await.map(|_| ())
            .map_err(|e| ErrorMessage::DatabaseError(e.to_string()))
    }
}

impl From<user::Model> for UserData {
    fn from(value: user::Model) -> Self {
        Self {
            uid: value.id,
            username: value.username,
            admin: value.admin,
            source: value.user_source,
        }
    }
}