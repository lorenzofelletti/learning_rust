use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// Matrix Multiplication Benchmark
pub struct Cli {
    #[arg(default_value_t = 128)]
    /// Size of the matrix
    pub size: usize,

    #[arg(short, long, default_value_t = 5)]
    /// Number of iterations to run the benchmark
    pub iterations: usize,

    #[arg(short, long, default_value_t = 4)]
    /// Number of threads to use for parallel matrix multiplication
    pub threads: usize,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    /// Only run parallel matrix multiplication
    pub parallel_only: bool,

    #[command(subcommand)]
    pub subcommands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "os_threads")]
    /// Print the number of available OS threads
    OsThreads,
}
