#[derive(Debug, PartialEq)]
/// Enum to represent the errors that can occur during the sanitization of the matrices
pub enum SanitizeError {
    EmptyMatrix(String),
    NotSquareMatrix(String),
    NotSameSize,
}

#[derive(Debug, PartialEq)]
/// Enum to represent the result of the sanitization of the matrices
///
/// # Ok
///
/// The matrices are valid
///
/// # NotOk
///
/// The matrices are not valid
pub enum SanitizeResult {
    Ok,
    NotOk(SanitizeError),
}

fn is_matrix_square(a: &Vec<Vec<i32>>, matrix_name: &str) -> SanitizeResult {
    if a.is_empty() {
        return SanitizeResult::NotOk(SanitizeError::EmptyMatrix(matrix_name.to_string()));
    }

    let row_length = a.len();
    
    let mut is_square = true;

    for row in a {
        if row.len() != row_length {
            is_square = false;
            break;
        }
    }

    match is_square {
        true => SanitizeResult::Ok,
        false => SanitizeResult::NotOk(SanitizeError::NotSquareMatrix(matrix_name.to_string())),
    }
}

fn are_square_matrices_same_size(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
    a.len() == b.len()
}

/// Sanitizes the matrices
///
/// # Arguments
///
/// * `a` - The first matrix
/// * `b` - The second matrix
///
/// # Returns
///
/// A `SanitizeResult` enum
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_3x3() -> Vec<Vec<i32>> {
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    }

    fn get_2x2() -> Vec<Vec<i32>> {
        vec![vec![1, 2], vec![3, 4]]
    }

    fn get_3x2() -> Vec<Vec<i32>> {
        vec![vec![1, 2], vec![3, 4], vec![5, 6]]
    }

    fn get_empty() -> Vec<Vec<i32>> {
        vec![]
    }

    fn get_row_of_different_length() -> Vec<Vec<i32>> {
        vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8]]
    }

    #[test]
    fn test_is_matrix_empty() {
        let a = get_3x3();
        let b = get_empty();

        assert_eq!(is_matrix_square(&a, "A"), SanitizeResult::Ok);
        assert_eq!(
            is_matrix_square(&b, "B"),
            SanitizeResult::NotOk(SanitizeError::EmptyMatrix("B".to_string()))
        );
    }

    #[test]
    fn test_is_matrix_square() {
        let a = get_3x3();
        let b = get_3x2();
        let c = get_row_of_different_length();

        assert_eq!(is_matrix_square(&a, "A"), SanitizeResult::Ok);
        assert_eq!(
            is_matrix_square(&b, "B"),
            SanitizeResult::NotOk(SanitizeError::NotSquareMatrix("B".to_string()))
        );
        assert_eq!(
            is_matrix_square(&c, "C"),
            SanitizeResult::NotOk(SanitizeError::NotSquareMatrix("C".to_string()))
        );
    }

    #[test]
    fn test_are_square_matrices_same_size() {
        let a = get_3x3();
        let b = get_3x3();
        let c = get_2x2();

        assert_eq!(are_square_matrices_same_size(&a, &b), true);
        assert_eq!(are_square_matrices_same_size(&a, &c), false);
    }

    #[test]
    fn test_sanitize_matrices() {
        let a = get_3x3();
        let b = get_3x3();
        let c = get_2x2();

        assert_eq!(sanitize_matrices(&a, &b), SanitizeResult::Ok);
        assert_eq!(
            sanitize_matrices(&a, &c),
            SanitizeResult::NotOk(SanitizeError::NotSameSize)
        );
    }
}
