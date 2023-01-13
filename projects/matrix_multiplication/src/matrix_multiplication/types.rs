/// Struct holding pointers to `MatrixRowPtr` type
pub struct SquareMatrixPtr(pub Vec<MatrixRowPtr>);

impl SquareMatrixPtr {
    /// Create new `SquareMatrixPtr` from `Vec<Vec<i32>>`
    pub fn new(matrix: &Vec<Vec<i32>>) -> SquareMatrixPtr {
        let mut matrix_ptr = Vec::new();

        for row in matrix {
            matrix_ptr.push(MatrixRowPtr(row.as_ptr()));
        }

        SquareMatrixPtr(matrix_ptr)
    }

    /// Get row by index
    /// 
    /// # Panics
    /// 
    /// Panics if `row` is out of bounds
    pub fn get_row(&self, row: usize) -> &MatrixRowPtr {
        let size = self.0.len();
        if row > size {
            panic!("Row index out of bounds");
        }

        &self.0[row]
    }
}

unsafe impl Send for SquareMatrixPtr {}

// incapsule *mut i32 into custom type implementing send
pub struct MatrixRowMutPtr(pub *mut i32);

/// Struct holding mutable pointers to `i32` type
/// It represents a row of a matrix that can be modified
impl MatrixRowMutPtr {
    /// Get value by index
    /// 
    /// # Arguments
    /// 
    /// * `offset` - index of value
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it dereferences a raw pointer, and it
    /// is the caller's responsibility to ensure that the pointer is valid.
    pub unsafe fn add(&mut self, offset: usize) -> &mut i32 {
        &mut *self.0.add(offset)
    }
}

unsafe impl Send for MatrixRowMutPtr {}

/// Struct holding pointers to `i32` type
/// It represents a row of a matrix
/// 
/// 
pub struct MatrixRowPtr(pub *const i32);

impl MatrixRowPtr {
    /// Get value by index
    /// 
    /// # Arguments
    /// 
    /// * `offset` - index of value
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it dereferences a raw pointer, and it
    /// is the caller's responsibility to ensure that the pointer is valid.
    pub unsafe fn add(&self, offset: usize) -> &i32 {
        &*self.0.add(offset)
    }
}

unsafe impl Send for MatrixRowPtr {}

#[cfg(test)]
mod tests {
    use super::SquareMatrixPtr;

    #[test]
    fn test_square_matrix_ptr() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let a_ptr = SquareMatrixPtr::new(&a);

        unsafe {
            let a_0 = a_ptr.get_row(0);
            assert_eq!(*a_0.add(0), 1);
            assert_eq!(*a_0.add(1), 2);
            assert_eq!(*a_0.add(2), 3);

            let a_1 = a_ptr.get_row(1);
            assert_eq!(*a_1.add(0), 4);
            assert_eq!(*a_1.add(1), 5);
            assert_eq!(*a_1.add(2), 6);

            let a_2 = a_ptr.get_row(2);
            assert_eq!(*a_2.add(0), 7);
            assert_eq!(*a_2.add(1), 8);
            assert_eq!(*a_2.add(2), 9);
        }
    }
}
