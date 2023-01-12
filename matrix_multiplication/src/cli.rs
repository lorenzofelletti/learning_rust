use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(default_value_t = 128)]
    pub size: usize,

    #[arg(short, long, default_value_t = 5)]
    pub iterations: usize,

    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,

    #[command(subcommand)]
    pub subcommands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "os_threads")]
    OsThreads,
}