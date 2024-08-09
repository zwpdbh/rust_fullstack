use ::entity::acstor::prelude::*;
use ::entity::acstor::storage_type;
use ::entity::user;
use ::entity::user::Entity as User;

use sea_orm::{DbConn, DbErr, EntityTrait, Set};

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            username: Set(form_data.username.to_owned()),
            address: Set(form_data.address.to_owned()),
            email: Set(form_data.email.to_owned()),
            age: Set(form_data.age.to_owned()),
            ..Default::default()
        };

        let res = User::insert(active_model).exec(db).await?;

        Ok(user::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn create_storage_type(
        db: &DbConn,
        form_data: storage_type::Model,
    ) -> Result<storage_type::Model, DbErr> {
        let active_model = storage_type::ActiveModel {
            name: Set(form_data.name.to_string()),
            ..Default::default()
        };

        let res = StorageType::insert(active_model).exec(db).await?;
        Ok(storage_type::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }
}
