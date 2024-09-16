use clap::{Parser, Subcommand};

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
    Sql {
        #[clap(subcommand)]
        case: ExCase,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ExCase {
    Case01 {
        #[arg(short, long)]
        name: String,
    },
    MigrateBookstore,
}
