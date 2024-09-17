use sqlx::{pool, FromRow};
use sqlx::{Postgres, Transaction};
use std::error::Error;

#[derive(Debug, FromRow)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl Book {
    pub async fn create(&self, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let q = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
        sqlx::query(q)
            .bind(&self.title)
            .bind(&self.author)
            .bind(&self.isbn)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn update(&self, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let q = "update book set title = $1, author = $2 where isbn = $3";
        sqlx::query(q)
            .bind(&self.title)
            .bind(&self.author)
            .bind(&self.isbn)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
        let q = "SELECT title, author, isbn FROM book";
        let query = sqlx::query_as::<_, Book>(q);
        let books = query.fetch_all(conn).await?;
        Ok(books)
    }

    pub async fn insert_book(book: &Book, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let mut txn: Transaction<'_, Postgres> = conn.begin().await?;

        let author_q = r"INSERT INTO author (name) VALUES ($1) RETURNING id";

        let book_q = r"
            INSERT INTO book (title, author_id, isbn) 
            VALUES ($1, $2, $3)";

        let author_id: (i64,) = sqlx::query_as(author_q)
            .bind(&book.author)
            .fetch_one(&mut *txn)
            .await?;

        let _ = sqlx::query(book_q)
            .bind(&book.title)
            .bind(&author_id.0)
            .bind(&book.isbn)
            .execute(&mut *txn)
            .await?;

        let _ = txn.commit().await?;

        Ok(())
    }
}
