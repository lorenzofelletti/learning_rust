use rand;
use thread_pool::ThreadPool;

use crate::thread_pool;

// incapsule *mut i32 into custom type implementing send
struct CMatrixElement(*mut i32);

impl CMatrixElement {
    unsafe fn add(&mut self, offset: usize) -> &mut i32 {
        &mut *self.0.add(offset)
    }
}

unsafe impl Send for CMatrixElement {}

struct AMatrixElement(*const i32);

impl AMatrixElement {
    unsafe fn add(&self, offset: usize) -> &i32 {
        &*self.0.add(offset)
    }
}

unsafe impl Send for AMatrixElement {}

pub fn generate_square_matrix_of_size(size: usize, random_values: bool) -> Vec<Vec<i32>> {
    let mut matrix = Vec::with_capacity(size);

    for _ in 0..size {
        let mut row = Vec::with_capacity(size);
        for _ in 0..size {
            if random_values {
                // random between -10 and 10
                row.push(rand::random::<i32>() % 20 - 10);
                //row.push(rand::random::<i32>());
            } else {
                row.push(0);
            }
        }
        matrix.push(row);
    }

    matrix
}

fn is_matrix_empty(a: &Vec<Vec<i32>>) -> bool {
    a.is_empty()
}

fn is_matrix_square(a: &Vec<Vec<i32>>) -> bool {
    if a.len() == 0 {
        return false;
    }

    let row_length = a.len();
    let col_length = a[0].len();

    row_length == col_length
}

fn are_matrices_square_and_same_size(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
    is_matrix_square(a) && is_matrix_square(b) && a.len() == b.len()
}

pub fn matrix_multiplication_sequential_ijk(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if is_matrix_empty(a) || is_matrix_empty(b) {
        return Vec::new();
    }

    if !are_matrices_square_and_same_size(a, b) {
        panic!("Matrices must be square and have the same size!");
    }

    let size = a.len();

    let mut c = generate_square_matrix_of_size(size, false);

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

pub fn matrix_multiplication_sequential_ikj(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if is_matrix_empty(a) || is_matrix_empty(b) {
        return Vec::new();
    }

    if !are_matrices_square_and_same_size(a, b) {
        panic!("Matrices must be square and have the same size!");
    }

    let size = a.len();

    let mut c = generate_square_matrix_of_size(size, false);

    for i in 0..size {
        for k in 0..size {
            for j in 0..size {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

pub fn matrix_multiplication_parallel_i_loop(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
    preferred_number_of_threads: usize,
) -> Vec<Vec<i32>> {
    if is_matrix_empty(a) || is_matrix_empty(b) {
        return Vec::new();
    }

    if !are_matrices_square_and_same_size(a, b) {
        panic!("Matrices must be square and have the same size!");
    }

    let size = a.len();

    let mut c = generate_square_matrix_of_size(size, false);

    let pool = ThreadPool::new(preferred_number_of_threads);

    let flatten_b = b.iter().flatten().cloned().collect::<Vec<_>>();

    for i in 0..size {
        let a_i = AMatrixElement(a[i].as_ptr());
        let mut c_i = CMatrixElement(c[i].as_mut_ptr());
        let flatten_b = AMatrixElement(flatten_b.as_ptr());

        unsafe {
            pool.execute(move || {
                for j in 0..size {
                    for k in 0..size {
                        *c_i.add(j) += *a_i.add(k) * *flatten_b.add(k * size + j);
                    }
                }
            });
        }
    }

    drop(pool);

    c
}
