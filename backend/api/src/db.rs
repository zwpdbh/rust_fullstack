use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;

pub async fn setup_db() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let db_user = "postgres";
    let db_password = "postgres";
    let db_name = "myapp";
    let url = format!("postgres://{db_user}:{db_password}@localhost:5432/{db_name}");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub async fn test_db_setup() {
        let db = setup_db().await.unwrap();
        let res = sqlx::query("SELECT 1 + 1 as sum")
            .fetch_one(&db)
            .await
            .unwrap();
        let sum: i32 = res.get("sum");
        println!("1+ 1 = {sum}");
    }

    #[test]
    pub fn case01() {
        task::block_in_place(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let _ = test_db_setup().await;
            })
        })
    }
}
