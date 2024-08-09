# How to layout

Separating API and service layers in a project, especially in a SeaOrm GraphQL project, follows a common architectural pattern that helps maintain a clean and organized codebase. Each layer has a distinct role and responsibility, contributing to better separation of concerns, scalability, and maintainability. Let's explore the purpose and responsibilities of each layer in your project:

### 1. Entity Layer

**Purpose**: 
- This layer contains the definitions of your database entities, using SeaOrm.
- It includes the schema for the database tables, relations between entities, and any model-specific logic.

**Responsibilities**:
- Define the data structure and schema.
- Handle data mapping between the database and the application.

**Example**:
```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl ActiveModelBehavior for ActiveModel {}
```

### 2. Migration Layer

**Purpose**: 
- This layer contains the database migration scripts.
- It is responsible for creating, altering, and managing the database schema over time.

**Responsibilities**:
- Define database migrations (e.g., creating tables, adding columns).
- Ensure the database schema evolves in a controlled manner.

**Example**:
```rust
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() -> Result<(), DbErr> {
    let db = Database::connect("DATABASE_URL").await?;
    migration().exec(&db).await?;
    Ok(())
}

#[migration]
pub fn migration() -> Migration {
    Migration::new(|m| {
        m.create_table("user", |t| {
            t.add_column("id", ColumnType::Integer);
            t.add_column("name", ColumnType::String(None));
            t.add_column("email", ColumnType::String(None));
            t.set_primary_key(vec!["id"]);
        })
    })
}
```

### 3. Service Layer

**Purpose**: 
- This layer contains the business logic of your application.
- It interacts with the entity layer to perform operations like CRUD (Create, Read, Update, Delete).

**Responsibilities**:
- Implement business rules and processes.
- Provide a layer of abstraction over the raw data access operations.
- Handle complex business operations and transformations.

**Example**:
```rust
pub struct UserService;

impl UserService {
    pub async fn create_user(
        db: &DatabaseConnection, 
        new_user: NewUser
    ) -> Result<Model, DbErr> {
        let user = ActiveModel {
            name: Set(new_user.name),
            email: Set(new_user.email),
            ..Default::default()
        };
        user.insert(db).await
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection, 
        user_id: i32
    ) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(user_id).one(db).await
    }
}
```

### 4. API Layer

**Purpose**: 
- This layer exposes the application's functionality to the outside world, typically through GraphQL or RESTful APIs.
- It handles incoming requests, interacts with the service layer, and returns responses.

**Responsibilities**:
- Define API endpoints and resolvers.
- Validate and process incoming requests.
- Coordinate with the service layer to perform business operations.
- Format and return the responses.

**Example**:
```rust
use async_graphql::{Context, Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(
        &self, 
        ctx: &Context<'_>, 
        id: i32
    ) -> Result<User, async_graphql::Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let user_service = UserService;
        user_service.get_user_by_id(db, id)
            .await
            .map_err(|e| e.into())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(
        &self, 
        ctx: &Context<'_>, 
        input: NewUser
    ) -> Result<User, async_graphql::Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let user_service = UserService;
        user_service.create_user(db, input)
            .await
            .map_err(|e| e.into())
    }
}
```

### Benefits of Separation

1. **Separation of Concerns**: Each layer has a distinct responsibility, making the code easier to understand and maintain.
2. **Reusability**: Services can be reused across different parts of the application or even different applications.
3. **Scalability**: It’s easier to scale different parts of the application independently.
4. **Testability**: Individual layers can be tested in isolation, improving test coverage and reliability.
5. **Flexibility**: Changes in one layer (e.g., database schema) require minimal changes in other layers, making the application more flexible to change.

### Summary

- **Entity Layer**: Defines the database schema and data structure.
- **Migration Layer**: Manages database schema changes over time.
- **Service Layer**: Implements business logic and interacts with the entity layer.
- **API Layer**: Exposes the application's functionality through GraphQL or RESTful APIs and handles request-response cycles.

By organizing your project in this way, you create a modular and maintainable codebase that can grow and evolve more efficiently.