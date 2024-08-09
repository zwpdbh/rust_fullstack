use ::entity::user;
use sea_orm::DatabaseBackend;
use sea_orm::DatabaseConnection;
use sea_orm::MockDatabase;
use sea_orm::MockExecResult;
// use sea_orm::*;

///  cargo test -p service -F mock --test mock -- --nocapture
/// And configure rust-analyzer by: "rust-analyzer.cargo.features": ["mock"] to have VS Code recognize and analyze code under conditional compilation attributes
#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    // Create a new Mock database connection
    MockDatabase::new(DatabaseBackend::Postgres)
        // Add query results
        .append_query_results(vec![
            // User 1
            [user::Model {
                id: 1,
                username: "张三".to_string(),
                email: "zhangsan@example.com".to_string(),
                address: Some("河南省郑州市".to_string()),
                age: 25,
            }],
            // User 2
            [user::Model {
                id: 2,
                username: "李四".to_string(),
                email: "lisi@example.com".to_string(),
                address: Some("广东省广州市".to_string()),
                age: 30,
            }],
            // User 3
            [user::Model {
                id: 3,
                username: "王五".to_string(),
                email: "wangwu@example.com".to_string(),
                address: Some("上海市".to_string()),
                age: 22,
            }],
            [user::Model {
                id: 4,
                username: "张三".to_string(),
                email: "zhangsan@example.com".to_string(),
                address: Some("河南省郑州市".to_string()),
                age: 25,
            }],
            [user::Model {
                id: 4,
                username: "李四".to_string(),
                email: "lisi@qq.com".to_string(),
                address: Some("地球村".to_string()),
                age: 0,
            }],
            [user::Model {
                id: 6,
                username: "张三6".to_string(),
                email: "zhangsan6@example.com".to_string(),
                address: Some("河南省郑州市".to_string()),
                age: 6,
            }],
        ])
        // Add execution results
        .append_exec_results([MockExecResult {
            last_insert_id: 4,
            rows_affected: 1,
        }])
        // Convert the Mock database connection to the DatabaseConnection type
        .into_connection()
}
