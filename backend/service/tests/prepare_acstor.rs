use ::entity::user;
use ::entity::user::Entity as User;
use sea_orm::DatabaseBackend;
use sea_orm::DatabaseConnection;
use sea_orm::MockDatabase;
// use sea_orm::MockExecResult;
// use sea_orm::*;

///  cargo test -p service -F mock --test mock -- --nocapture
/// And configure rust-analyzer by: "rust-analyzer.cargo.features": ["mock"] to have VS Code recognize and analyze code under conditional compilation attributes
#[cfg(feature = "mock")]
pub async fn prepare_mock_db() -> DatabaseConnection {
    // Create a new Mock database connection

    use sea_orm::{EntityTrait, Set};
    let mock_db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    // // Add query results
    // mock_db.append_query_results(vec![
    //     // User 1
    //     [user::Model {
    //         id: 2,
    //         username: "user01".to_string(),
    //         email: "zhangsan@example.com".to_string(),
    //         address: Some("河南省郑州市".to_string()),
    //         age: 25,
    //     }],
    // ]);

    // mock_db.
    let user_model = user::ActiveModel {
        username: Set("user01".to_string()),
        ..Default::default()
    };

    let _ = User::insert(user_model).exec(&mock_db).await.unwrap();

    // Convert the Mock database connection to the DatabaseConnection type
    mock_db
}
