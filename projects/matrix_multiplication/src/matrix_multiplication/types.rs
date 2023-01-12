// incapsule *mut i32 into custom type implementing send
pub struct MatrixRowMutPtr(pub *mut i32);

impl MatrixRowMutPtr {
    pub unsafe fn add(&mut self, offset: usize) -> &mut i32 {
        &mut *self.0.add(offset)
    }
}

unsafe impl Send for MatrixRowMutPtr {}

pub struct MatrixRowPtr(pub *const i32);

impl MatrixRowPtr {
    pub unsafe fn add(&self, offset: usize) -> &i32 {
        &*self.0.add(offset)
    }
}

unsafe impl Send for MatrixRowPtr {}
