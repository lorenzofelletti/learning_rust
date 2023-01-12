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
