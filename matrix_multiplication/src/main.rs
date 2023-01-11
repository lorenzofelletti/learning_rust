use std::time::Instant;

use crate::matrix_multiplication::{generate_square_matrix_of_size, matrix_multiplication_sequential_ijk, matrix_multiplication_sequential_ikj, matrix_multiplication_parallel_i_loop};

mod matrix_multiplication;
mod thread_pool;

fn main() {
    let n = 256;

    // collect execution times for different matrix multiplication methods

    let mut sequential_ijk_times = Vec::new();
    let mut sequential_ikj_times = Vec::new();
    let mut parallel_ijk_times = Vec::new();

    for _ in 0..5 {
        let a = generate_square_matrix_of_size(n, true);
        let b = generate_square_matrix_of_size(n, true);

        //println!("Multplying:");
        //println!("A: {:?}", a);
        //println!("B: {:?}", b);

        let start = Instant::now();
        let _c = matrix_multiplication_sequential_ijk(&a, &b);
        let end = Instant::now();
        sequential_ijk_times.push(end.duration_since(start).as_millis());

        //println!("ijk: {:?}", _c);
        println!("finished sequential ijk");

        let start = Instant::now();
        let _c = matrix_multiplication_sequential_ikj(&a, &b);
        let end = Instant::now();
        sequential_ikj_times.push(end.duration_since(start).as_millis());

        //println!("ikj: {:?}", _c);
        println!("finished sequential ikj");

        let start = Instant::now();
        let _c = matrix_multiplication_parallel_i_loop(&a, &b, 4);
        let end = Instant::now();
        parallel_ijk_times.push(end.duration_since(start).as_millis());

        //println!("parallel ijk: {:?}", _c);
        println!("finished parallel ijk");
    }

    // calculate average execution times

    let sequential_ijk_average_time =
        sequential_ijk_times.iter().sum::<u128>() as f64 / sequential_ijk_times.len() as f64;
    let sequential_ikj_average_time =
        sequential_ikj_times.iter().sum::<u128>() as f64 / sequential_ikj_times.len() as f64;
    let parallel_ijk_average_time =
        parallel_ijk_times.iter().sum::<u128>() as f64 / parallel_ijk_times.len() as f64;

    // print results

    println!(
        "Sequential ijk average time: {} ms",
        sequential_ijk_average_time
    );
    println!(
        "Sequential ikj average time: {} ms",
        sequential_ikj_average_time
    );
    println!(
        "Parallel ijk average time: {} ms",
        parallel_ijk_average_time
    );
}
