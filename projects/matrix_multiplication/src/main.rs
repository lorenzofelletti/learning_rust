use std::{thread, time::Instant};

use clap::Parser;
use log::debug;

use crate::{
    cli::Cli,
    matrix_multiplication::{
        generate_square_matrix_of_size, matrix_multiplication_parallel_i_loop,
        matrix_multiplication_sequential_ijk, matrix_multiplication_sequential_ikj,
    },
};

mod cli;
mod matrix_multiplication;
mod thread_pool;

fn matrix_multiplication_benchmark(cli: &Cli) {
    let n = cli.size;
    let iterations = cli.iterations;
    let threads = cli.threads;
    let parallel_only: bool = cli.parallel_only;

    // collect execution times for different matrix multiplication methods

    let mut sequential_ijk_times = Vec::new();
    let mut sequential_ikj_times = Vec::new();
    let mut parallel_ijk_times = Vec::new();

    println!("Welcome to Matrix Multiplication Benchmark!");
    println!("Matrix size: {}", n);
    println!("Number of threads: {}", threads);
    println!("Number of iterations: {}", iterations);
    println!("Parallel only: {}", parallel_only);

    for i in 0..iterations {
        println!("starting iteration {} of {}", i + 1, iterations);

        let a = generate_square_matrix_of_size(n, true);
        let b = generate_square_matrix_of_size(n, true);

        if parallel_only == false {
            let start = Instant::now();
            let _c = matrix_multiplication_sequential_ijk(&a, &b);
            let end = Instant::now();
            sequential_ijk_times.push(end.duration_since(start).as_millis());

            println!("finished sequential ijk");
            debug!("finished sequential ijk");

            let start = Instant::now();
            let _c = matrix_multiplication_sequential_ikj(&a, &b);
            let end = Instant::now();
            sequential_ikj_times.push(end.duration_since(start).as_millis());

            println!("finished sequential ikj");
            debug!("finished sequential ikj");
        }

        let start = Instant::now();
        let _c = matrix_multiplication_parallel_i_loop(&a, &b, threads);
        let end = Instant::now();
        parallel_ijk_times.push(end.duration_since(start).as_millis());

        println!("finished parallel i-loop");
        debug!("finished parallel i-loop");

        println!();
    }

    // calculate average execution times

    let sequential_ijk_average = sequential_ijk_times.iter().sum::<u128>() / iterations as u128;
    let sequential_ikj_average = sequential_ikj_times.iter().sum::<u128>() / iterations as u128;
    let parallel_ijk_average = parallel_ijk_times.iter().sum::<u128>() / iterations as u128;

    // print results

    println!("Benchmark Results");
    println!("sequential ijk average: {} ms", sequential_ijk_average);
    println!("sequential ikj average: {} ms", sequential_ikj_average);
    println!("parallel ijk average: {} ms", parallel_ijk_average);
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommands {
        Some(cli::Commands::OsThreads) => {
            print!(
                "number of os threads: {}",
                thread::available_parallelism().unwrap()
            );
        }
        None => {
            matrix_multiplication_benchmark(&cli);
        }
    }
}
