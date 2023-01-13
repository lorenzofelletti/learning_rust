
/// Generates a square matrix of size `size` filled with zeros or random values between -10 and 10
/// 
/// # Arguments
/// 
/// * `size` - The size of the matrix
/// * `random_values` - If true, the matrix will be filled with random values between -10 and 10
/// 
/// # Returns
/// 
/// A square matrix of size `size` as a `Vec<Vec<i32>>`
/// ```
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
