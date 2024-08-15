use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
struct Args {

}

#[derive(Subcommand)]
enum Command {
    Server,
    Migrate,
}