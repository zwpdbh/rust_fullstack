# How to test service

## How to run

```sh 
cargo test -p service -F mock --test user -- --nocapture
```

## How to enable vscode check mock code 

Edit VS Code configuration to recognize and analyze code under conditional compilation attributes:

```json
{
    "rust-analyzer.cargo.features": ["mock"]
}
```


## How to test service from ChatGPT 

To perform mock testing in the service layer of a SeaOrm project, you can use the `mockall` crate, which allows you to create mock objects for testing purposes. Mock testing helps you isolate the service layer and test its functionality without relying on the actual database or other dependencies.

### Steps to Perform Mock Testing in the Service Layer

1. **Add Dependencies**:
   Add `mockall`, `sea-orm`, and `tokio` (for asynchronous testing) to your `Cargo.toml` file:

   ```toml
   [dependencies]
   sea-orm = "0.9"
   tokio = { version = "1", features = ["full"] }

   [dev-dependencies]
   mockall = "0.10"
   ```

2. **Define the Service Trait**:
   Define a trait for your service layer. This trait will be implemented by both the real service and the mock service.

   ```rust
   use async_trait::async_trait;
   use sea_orm::DatabaseConnection;
   use sea_orm::DbErr;

   #[async_trait]
   pub trait UserServiceTrait {
       async fn create_user(&self, db: &DatabaseConnection, name: &str, email: &str) -> Result<(), DbErr>;
       async fn get_user_by_id(&self, db: &DatabaseConnection, user_id: i32) -> Result<Option<String>, DbErr>;
   }
   ```

3. **Implement the Trait in Your Service**:
   Implement the trait for your actual service:

   ```rust
   use sea_orm::entity::prelude::*;
   use sea_orm::ActiveModelTrait;

   pub struct UserService;

   #[async_trait::async_trait]
   impl UserServiceTrait for UserService {
       async fn create_user(&self, db: &DatabaseConnection, name: &str, email: &str) -> Result<(), DbErr> {
           let user = user::ActiveModel {
               name: Set(name.to_string()),
               email: Set(email.to_string()),
               ..Default::default()
           };
           user.insert(db).await?;
           Ok(())
       }

       async fn get_user_by_id(&self, db: &DatabaseConnection, user_id: i32) -> Result<Option<String>, DbErr> {
           let user = user::Entity::find_by_id(user_id).one(db).await?;
           Ok(user.map(|u| u.name))
       }
   }
   ```

4. **Create Mock Implementations**:
   Use `mockall` to create mock implementations of the service trait for testing:

   ```rust
   use mockall::{automock, mock};
   use async_trait::async_trait;

   #[automock]
   #[async_trait]
   pub trait UserServiceTrait {
       async fn create_user(&self, db: &DatabaseConnection, name: &str, email: &str) -> Result<(), DbErr>;
       async fn get_user_by_id(&self, db: &DatabaseConnection, user_id: i32) -> Result<Option<String>, DbErr>;
   }
   ```

5. **Write Tests Using Mocks**:
   Use the mock implementation in your tests to verify the behavior of your service layer:

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       use mockall::predicate::*;
       use sea_orm::DatabaseConnection;
       use sea_orm::DbErr;

       #[tokio::test]
       async fn test_create_user() {
           let mut mock = MockUserServiceTrait::new();
           let db = DatabaseConnection::default(); // Mock or real connection

           mock.expect_create_user()
               .with(predicate::eq(&db), predicate::eq("John Doe"), predicate::eq("john@example.com"))
               .returning(|_, _, _| Ok(()));

           let result = mock.create_user(&db, "John Doe", "john@example.com").await;
           assert!(result.is_ok());
       }

       #[tokio::test]
       async fn test_get_user_by_id() {
           let mut mock = MockUserServiceTrait::new();
           let db = DatabaseConnection::default(); // Mock or real connection

           mock.expect_get_user_by_id()
               .with(predicate::eq(&db), predicate::eq(1))
               .returning(|_, _| Ok(Some("John Doe".to_string())));

           let result = mock.get_user_by_id(&db, 1).await;
           assert_eq!(result.unwrap(), Some("John Doe".to_string()));
       }
   }
   ```

### Summary

1. **Define a Service Trait**: Create a trait that outlines the methods in your service layer.
2. **Implement the Trait**: Implement the trait for your actual service.
3. **Create Mock Implementations**: Use the `mockall` crate to create mock implementations of the service trait.
4. **Write Tests**: Use the mock implementations in your tests to verify the service layer's behavior.

This approach allows you to test the service layer in isolation, ensuring that your business logic is correct without the need for a database connection or other dependencies.

## Service Cargo.toml explain

In your `Cargo.toml` file, the sections under `[[test]]` are specifying test targets, and the `[features]` section is defining conditional compilation features. Here's a detailed explanation of each part:

### `[[test]]` Sections

The `[[test]]` sections define individual integration test targets. Each section specifies a separate test target and its required features.

1. **`[[test]]` for `user`**:

    ```toml
    [[test]]
    name = "user"
    required-features = ["mock"]
    ```

    This block defines an integration test named `user`. The test will be compiled and run when you execute `cargo test --test user`. It requires the `mock` feature to be enabled.

2. **`[[test]]` for `acstor`**:

    ```toml
    [[test]]
    name = "acstor"
    required-features = ["mock"]
    ```

    This block defines another integration test named `acstor`. Similar to the `user` test, it will be compiled and run when you execute `cargo test --test acstor`. It also requires the `mock` feature to be enabled.

### `[features]` Section

The `[features]` section defines conditional compilation features for your crate.

```toml
[features]
mock = ["sea-orm/mock"]
```

- **`mock`**: This feature, when enabled, includes the `sea-orm/mock` feature. This is useful for testing purposes where you want to mock database interactions using SeaORM's mock feature.

### How They Work Together

- When you run `cargo test -p service -F mock --test user`, Cargo compiles the `service` package, enabling the `mock` feature, and runs the tests defined in the `tests/user.rs` file.
- Similarly, when you run `cargo test -p service -F mock --test acstor`, Cargo compiles the `service` package with the `mock` feature enabled and runs the tests defined in the `tests/acstor.rs` file.

### Example Usage

Assuming you have the following test files:

- `tests/user.rs`
- `tests/acstor.rs`

Each file should contain test functions relevant to its context, and they will be compiled and executed based on the corresponding `[[test]]` section in the `Cargo.toml` file.

**Example `tests/user.rs`:**

```rust
use sea_orm::DatabaseConnection;
use tokio;

#[tokio::test]
async fn test_user_creation() {
    // Test logic for user creation
    let db = DatabaseConnection::connect("your_database_url").await.unwrap();
    // Assume you have some user service logic to test
    let result = user_service::create_user(&db, "John Doe").await;
    assert!(result.is_ok());
}
```

**Example `tests/acstor.rs`:**

```rust
use sea_orm::DatabaseConnection;
use tokio;

#[tokio::test]
async fn test_acstor_functionality() {
    // Test logic for acstor functionality
    let db = DatabaseConnection::connect("your_database_url").await.unwrap();
    // Assume you have some acstor service logic to test
    let result = acstor_service::some_functionality(&db).await;
    assert!(result.is_ok());
}
```

### Running the Tests

To run the tests, you would use the following commands:

- For the `user` test:

    ```sh
    cargo test -p service -F mock --test user -- --nocapture
    ```

- For the `acstor` test:

    ```sh
    cargo test -p service -F mock --test acstor -- --nocapture
    ```

### Summary

- **`[[test]]` Sections**: Define individual integration test targets and their required features.
- **`[features]` Section**: Define conditional compilation features (e.g., `mock` for enabling SeaORM's mock feature).
- **Usage**: Run specific tests with the required features enabled using Cargo commands.

By organizing your tests and features in this way, you can selectively run and manage different integration tests with specific configurations and dependencies.

## References 

- [Mocking in Rust: Mockall and alternatives](https://blog.logrocket.com/mocking-rust-mockall-alternatives/)