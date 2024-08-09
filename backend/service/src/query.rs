use ::entity::user;
use sea_orm::*;
use user::Entity as User;

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn get_all_users(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(db).await
    }
}
