use crate::db::setup_db;
use crate::models::*;
use crate::run;
use clap::{Parser, Subcommand};
use std::error::Error;
use tracer::info;

#[derive(Parser, Debug)]
#[clap(author = "zhaowei", version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    Serve {
        #[arg(short, long)]
        port: u16,
    },
    Bookstore {
        #[clap(subcommand)]
        case: ExCase,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ExCase {
    CreateBook {
        #[arg(long)]
        title: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        isbn: String,
    },
    UpdateBook {
        #[arg(long)]
        title: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        isbn: String,
    },
    ReadBook,
    Migrate,
}

pub async fn process(args: Arguments) -> Result<(), Box<dyn Error>> {
    let db = setup_db().await?;

    match args.cmd {
        SubCommand::Serve { port } => {
            let _ = run(port).await.unwrap();
        }
        SubCommand::Bookstore { case } => match case {
            ExCase::Migrate => {
                let _ = sqlx::migrate!("./migrations/bookstore").run(&db).await?;
                info!("migration bookstore succeed");
            }
            ExCase::CreateBook {
                title,
                author,
                isbn,
            } => {
                let book = Book {
                    title: title.to_string(),
                    author: author.to_string(),
                    isbn: isbn.to_string(),
                };
                let _ = book.create(&db).await?;
            }
            ExCase::UpdateBook {
                title,
                author,
                isbn,
            } => {
                let book = Book {
                    title: title.to_string(),
                    author: author.to_string(),
                    isbn: isbn.to_string(),
                };
                let _ = book.update(&db).await?;
            }
            ExCase::ReadBook => {
                let books = Book::read(&db).await?;
                info!("{books:?}")
            }
        },
    }
    Ok(())
}
