#[derive(Debug)]
pub enum SanitizeError {
    EmptyMatrix(String),
    NotSquareMatrix(String),
    NotSameSize,
}

/// Enum to represent the result of the sanitization of the matrices
pub enum SanitizeResult {
    Ok,
    NotOk(SanitizeError),
}

fn is_matrix_square(a: &Vec<Vec<i32>>, matrix_name: &str) -> SanitizeResult {
    if a.is_empty() {
        return SanitizeResult::NotOk(SanitizeError::EmptyMatrix(matrix_name.to_string()));
    }

    let row_length = a.len();
    let col_length = a[0].len();

    match row_length == col_length {
        true => SanitizeResult::Ok,
        false => SanitizeResult::NotOk(SanitizeError::NotSquareMatrix(matrix_name.to_string())),
    }
}

fn are_square_matrices_same_size(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
    a.len() == b.len()
}

pub fn sanitize_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> SanitizeResult {
    match is_matrix_square(a, "A") {
        SanitizeResult::Ok => (),
        SanitizeResult::NotOk(e) => return SanitizeResult::NotOk(e),
    };

    match is_matrix_square(b, "B") {
        SanitizeResult::Ok => (),
        SanitizeResult::NotOk(e) => return SanitizeResult::NotOk(e),
    };

    match are_square_matrices_same_size(a, b) {
        true => SanitizeResult::Ok,
        false => SanitizeResult::NotOk(SanitizeError::NotSameSize),
    }
}
