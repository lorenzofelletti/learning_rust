#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Create a new rectangle with the given width and height.
    pub fn build_rectangle(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Create a new square with the given side length.
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    /// Calculates the area of the rectangle.
    ///
    /// # Arguments
    ///
    /// * `self` - The rectangle to calculate the area of.
    ///
    /// # Returns
    ///
    /// The area of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use rectangles::Rectangle;
    ///
    /// let rect = Rectangle::build_rectangle(30, 50);
    ///
    /// assert_eq!(rect.area(), 1500);
    /// ```
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Returns true if the rectangle has width 0.
    pub fn has_zero_width(&self) -> bool {
        self.width == 0
    }

    /// Returns true if the rectangle can hold the other rectangle.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Returns true if the rectangle has a square shape.
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
