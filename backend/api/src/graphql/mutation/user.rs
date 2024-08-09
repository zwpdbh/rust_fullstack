use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::user;
use service::Mutation;

use crate::db::Database;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub address: Option<String>,
    pub age: i32,
}

impl CreateUserInput {
    fn into_model_with_arbitrary_id(self) -> user::Model {
        user::Model {
            id: 0,
            username: self.username,
            email: self.email,
            address: self.address,
            age: self.age,
        }
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<user::Model> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Mutation::create_user(conn, input.into_model_with_arbitrary_id()).await?)
    }
}
