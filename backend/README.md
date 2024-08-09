# How to use SeaORM 

## Prerequisite 

```sh 
cargo install sea-orm-cli
```

Here, we follow schema first, then entity.

## Migration init

Create migration project in workspace:

```sh 
sea-orm-cli migrate init
```

- This will create a folder `migration` at the same path your execute this command. 
- It is a standard bin project. So, add it into your workspace like: 
  
  ```toml
    members = [
        # ...
        "backend/migration",
    ]
  ```
- Change the sample code from there to meet your need. And follow the automatically generated `README` in `migration` project.

## Generate entities 

1. Generate entity based on schema 

```sh 
sea-orm-cli generate entity --tables user -o entity/src --lib --with-serde both --model-extra-derives async_graphql::SimpleObject
```

- Table name is `user`, it is case sensitive.
- We generate the files into a new filder named `entity/src`. So, later we could use it as if it is created from `cargo new --lib`.
- So the current tree structure is: 

```sh
zw@zwpdbh:~/code/rust_programming/rust_fullstack/backend$ tree .
.
├── Cargo.toml
├── README.md
├── entity
│   └── src
│       ├── lib.rs
│       ├── prelude.rs
│       └── user.rs
├── migration
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_table.rs
│       └── main.rs
└── src
    ├── lib.rs
    └── query
        └── mod.rs

6 directories, 12 files
```


### Generate tables from database with specific tables

For example, here we only generate tables related with acstor

```sh
sea-orm-cli generate entity \
-o entity/src/acstor \
--tables key_feature,milestone,milestone_keyfeature,storage_type,storage_type_keyfeature,workload,workload_keyfeature \
--lib --with-serde both \
--model-extra-derives async_graphql::SimpleObject
```

2. Initialize and configure the entity lib 

```sh 
cd entity
cargo init 
```

It shows warning:
```txt
warning: compiling this new package may not work due to invalid workspace configuration

current package believes it's in a workspace when it's not:
current:   /home/zw/code/rust_programming/rust_fullstack/backend/entity/./Cargo.toml
workspace: /home/zw/code/rust_programming/rust_fullstack/Cargo.toml

this may be fixable by adding `backend/entity` to the `workspace.members` array of the manifest located at: /home/zw/code/rust_programming/rust_fullstack/Cargo.toml
Alternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest.
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

So, we edit the **workspace**'s `Cargo.toml` to be:
```toml
[workspace]
resolver = "2"

members = [
  # ...
  "backend",
  "backend/migration",
  "backend/entity",
]
```

Add dependencies by edit `entity`'s `Cargo.toml`:

```toml
[dependencies]
sea-orm = { version = "0.12", features = [
  "sqlx-postgres",
  "runtime-tokio-rustls",
  "macros",
  "with-uuid",
  "with-chrono",
  "with-json",
  "with-bigdecimal",
] }
async-graphql = "6.0.6"
serde_json = "1.0"
serde = { version = "1" }
```

3. Configure `backend` depends on `backend/entity`

```toml
[dependencies]
# ...
entity = { path = "./entity" }
```

## References

- [Axum+SeaORM+Async-graphql: Building a GraphQL Service from Scratch](https://dev.to/yexiyue/axumseaormasync-graphql-building-a-graphql-service-from-scratch-52kk)
- [Axum-GraphQL with SeaORM example app](https://github.com/SeaQL/sea-orm/blob/master/examples/graphql_example/README.md)

## Support both GraphQL and RESTful APIs in your backend service

To support both GraphQL and RESTful APIs in your backend service, you can follow a few common patterns that allow you to maintain a clean and organized codebase. Here’s a step-by-step approach to achieve this:

### 1. Design Your API Endpoints
- **GraphQL Endpoint**: Typically, you have a single endpoint for GraphQL, e.g., `/graphql`.
- **RESTful Endpoints**: Define RESTful endpoints following the REST conventions, e.g., `/users`, `/users/{id}`, etc.

### 2. Shared Business Logic
- Extract common business logic into separate services or modules that can be reused by both the GraphQL resolvers and REST handlers. This ensures that you don’t duplicate logic and maintain consistency across different API types.

### 3. Set Up the Router
- Use a router that can handle both GraphQL and RESTful routes. In Rust, with frameworks like `axum`, you can easily set up routes for both types of APIs.

### 4. Example Implementation

Here’s a basic example using `axum` to support both GraphQL and RESTful endpoints:

```rust
use axum::{
    routing::{get, post},
    Router,
    Extension,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use async_graphql::{Schema, EmptyMutation, EmptySubscription};

async fn graphql_handler(schema: Extension<Arc<Schema<MyQuery, EmptyMutation, EmptySubscription>>>) -> impl axum::response::IntoResponse {
    // Your GraphQL handler code here
}

async fn get_user_handler(/* params */) -> impl axum::response::IntoResponse {
    // Your RESTful handler code here
}

async fn create_user_handler(/* params */) -> impl axum::response::IntoResponse {
    // Your RESTful handler code here
}

pub async fn run(port: i32, db: DatabaseConnection) {
    let schema = Schema::build(MyQuery, EmptyMutation, EmptySubscription)
        .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/users", get(get_user_handler).post(create_user_handler))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(db));

    let address = format!("0.0.0.0:{}", port);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### 5. Handling Database Connections
- Ensure you are passing the database connection to both GraphQL and RESTful handlers, typically via a shared state or extensions.

### 6. Implementing GraphQL Resolvers and RESTful Handlers
- **GraphQL Resolvers**: Implement your GraphQL resolvers to handle queries and mutations.
- **RESTful Handlers**: Implement your RESTful handlers to handle HTTP methods like GET, POST, PUT, DELETE.

### Example of a Shared Business Logic Service

```rust
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub async fn get_user_by_id(&self, id: i32) -> Result<user::Model, DbErr> {
        // Common logic to get user by id
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<user::Model, DbErr> {
        // Common logic to create a user
    }
}

// GraphQL resolver using the service
pub struct MyQuery;

#[Object]
impl MyQuery {
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<user::Model, DbErr> {
        let user_service = ctx.data::<UserService>().unwrap();
        user_service.get_user_by_id(id).await
    }
}

// RESTful handler using the service
async fn get_user_handler(Extension(user_service): Extension<Arc<UserService>>, id: i32) -> impl IntoResponse {
    match user_service.get_user_by_id(id).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Error fetching user")),
    }
}
```

### Conclusion
By structuring your code to share business logic between GraphQL and RESTful endpoints and organizing your routes effectively, you can efficiently support both API types in a single backend service. This pattern ensures code reuse, consistency, and maintainability.