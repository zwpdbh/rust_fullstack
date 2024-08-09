use async_graphql::EmptySubscription;
use async_graphql::Schema;
use entity::async_graphql;

use crate::{
    db::Database,
    graphql::{mutation::Mutation, query::Query},
};

use migration::{Migrator, MigratorTrait};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Migrator::up(db.get_connection(), None).await.unwrap();

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
