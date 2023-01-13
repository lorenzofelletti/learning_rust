use log::error;

use thread_pool::ThreadPool;
use types::{MatrixRowMutPtr, MatrixRowPtr};

use crate::thread_pool;

use self::{
    generate::generate_square_matrix_of_size,
    sanitize::{sanitize_matrices, SanitizeResult},
    types::SquareMatrixPtr,
};

pub mod generate;
mod sanitize;
mod types;

pub fn matrix_multiplication_sequential_ijk(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
) -> Option<Vec<Vec<i32>>> {
    match sanitize_matrices(a, b) {
        SanitizeResult::Ok => (),
        SanitizeResult::NotOk(error) => {
            error!("Error: {:?}", error);
            return None;
        }
    };

    let size = a.len();

    let mut c = generate_square_matrix_of_size(size, false);

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Some(c)
}

pub fn matrix_multiplication_sequential_ikj(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
) -> Option<Vec<Vec<i32>>> {
    match sanitize_matrices(a, b) {
        SanitizeResult::Ok => (),
        SanitizeResult::NotOk(error) => {
            error!("Error: {:?}", error);
            return None;
        }
    };

    let size = a.len();

    let mut c = generate_square_matrix_of_size(size, false);

    for i in 0..size {
        for k in 0..size {
            for j in 0..size {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Some(c)
}

pub fn matrix_multiplication_parallel_i_loop(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
    preferred_number_of_threads: usize,
) -> Option<Vec<Vec<i32>>> {
    match sanitize_matrices(a, b) {
        SanitizeResult::Ok => (),
        SanitizeResult::NotOk(error) => {
            error!("Error: {:?}", error);
            return None;
        }
    };

    let size = a.len();

    let mut c = generate_square_matrix_of_size(size, false);

    let pool = ThreadPool::new(preferred_number_of_threads);

    for i in 0..size {
        let a_i = MatrixRowPtr(a[i].as_ptr());
        let mut c_i = MatrixRowMutPtr(c[i].as_mut_ptr());
        let b = SquareMatrixPtr::new(b);

        unsafe {
            pool.execute(move || {
                for k in 0..size {
                    let b_k = b.get_row(k);
                    for j in 0..size {
                        *c_i.add(j) += *a_i.add(k) * *b_k.add(j);
                    }
                }
            });
        }
    }

    ThreadPool::terminate(pool);

    Some(c)
}

#[cfg(test)]
mod tests {
    fn get_a() -> Vec<Vec<i32>> {
        vec![vec![1, 2], vec![3, 4]]
    }

    fn get_b() -> Vec<Vec<i32>> {
        vec![vec![5, 6], vec![7, 8]]
    }

    #[test]
    fn test_matrix_multiplication_sequential_ijk() {
        let a = get_a();
        let b = get_b();

        let c = super::matrix_multiplication_sequential_ijk(&a, &b).unwrap();

        assert_eq!(c, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn test_matrix_multiplication_sequential_ikj() {
        let a = get_a();
        let b = get_b();

        let c = super::matrix_multiplication_sequential_ikj(&a, &b).unwrap();

        assert_eq!(c, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn test_matrix_multiplication_parallel_i_loop() {
        let a = get_a();
        let b = get_b();

        let c = super::matrix_multiplication_parallel_i_loop(&a, &b, 2).unwrap();

        assert_eq!(c, vec![vec![19, 22], vec![43, 50]]);
    }
}
