/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/int-math-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub mod prelude;
mod test;

extern crate core;

use core::ops::{Add, Div, Mul, Sub};

/// A 2D vector with unsigned integer coordinates.
///
/// This struct is used to represent positions or dimensions in a grid where negative coordinates are not applicable.
///
/// # Examples
///
/// ```
/// use int_math::prelude::*;
///
/// let position = VectorU::new(5, 10);
/// println!("{:?}", position); // Output: VectorU { x: 5, y: 10 }
/// ```
///
/// # Fields
///
/// - `x`: The bottom-left x-coordinate (horizontal position) of the vector.
/// - `y`: The bottom-left y-coordinate (vertical position) of the vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorU {
    pub x: u32,
    pub y: u32,
}

impl VectorU {
    /// Creates a new `VectorU` instance with the specified coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: The bottom-left x-coordinate of the vector.
    /// - `y`: The bottom-left y-coordinate of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let vector = VectorU::new(5, 10);
    /// assert_eq!(vector.x, 5);
    /// assert_eq!(vector.y, 10);
    /// ```
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Add<VectorU> for VectorU {
    type Output = VectorU;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<VectorU> for VectorU {
    type Output = VectorU;

    fn sub(self, rhs: VectorU) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<u32> for VectorU {
    type Output = VectorU;

    fn div(self, rhs: u32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<u32> for VectorU {
    type Output = VectorU;

    fn mul(self, rhs: u32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(u32, u32)> for VectorU {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

/// A 2D vector with signed integer coordinates.
///
/// This struct is used to represent positions or dimensions where negative values are applicable.
///
/// # Examples
///
/// ```
/// use int_math::prelude::*;
///
/// let position = VectorI::new(-5, 10);
/// println!("{:?}", position); // Output: VectorI { x: -5, y: 10 }
/// ```
///
/// # Fields
///
/// - `x`: The bottom-left x-coordinate (horizontal position) of the vector.
/// - `y`: The bottom-left y-coordinate (vertical position) of the vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VectorI {
    pub x: i32,
    pub y: i32,
}

impl VectorI {
    /// Creates a new `VectorI` instance with the specified coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: The bottom-left x-coordinate of the vector.
    /// - `y`: The bottom-left y-coordinate of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let vector = VectorI::new(-5, 10);
    /// assert_eq!(vector.x, -5);
    /// assert_eq!(vector.y, 10);
    /// ```
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<VectorI> for VectorI {
    type Output = VectorI;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<VectorI> for VectorI {
    type Output = VectorI;

    fn sub(self, rhs: VectorI) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i32> for VectorI {
    type Output = VectorI;

    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<i32> for VectorI {
    type Output = VectorI;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(i32, i32)> for VectorI {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

/// A rectangle with unsigned integer dimensions.
///
/// This struct represents a rectangle on a 2D grid with its position and size.
///
/// # Examples
///
/// ```
/// use int_math::prelude::*;
///
/// let rect = RectU::new(10, 20, 30, 40);
/// println!("{:?}", rect); // Output: RectU { position: VectorU { x: 10, y: 20 }, size: VectorU { x: 30, y: 40 } }
/// ```
///
/// # Fields
///
/// - `position`: The lower-left corner of the rectangle as a `VectorU`.
/// - `size`: The width and height of the rectangle as a `VectorU`.
#[derive(Debug, Clone, Copy)]
pub struct RectU {
    pub position: VectorU,
    pub size: VectorU,
}

impl RectU {
    /// Creates a new `RectU` instance with the specified position and size.
    ///
    /// # Arguments
    ///
    /// - `x`: The x-coordinate of the bottom-left corner.
    /// - `y`: The y-coordinate of the bottom-left corner.
    /// - `width`: The width of the rectangle.
    /// - `height`: The height of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = RectU::new(10, 20, 30, 40);
    /// assert_eq!(rect.position.x, 10);
    /// assert_eq!(rect.position.y, 20);
    /// assert_eq!(rect.size.x, 30);
    /// assert_eq!(rect.size.y, 40);
    /// ```
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            position: VectorU { x, y },
            size: VectorU {
                x: width,
                y: height,
            },
        }
    }

    /// Creates a new `RectU` instance with the specified position and size.
    ///
    /// # Arguments
    ///
    /// - `position`: The bottom-left corner of the rectangle as a `VectorU`.
    /// - `size`: The width and height of the rectangle as a `VectorU`.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let position = VectorU::new(10, 20);
    /// let size = VectorU::new(30, 40);
    /// let rect = RectU::with_position_and_size(position, size);
    /// assert_eq!(rect.position, position);
    /// assert_eq!(rect.size, size);
    /// ```
    pub fn with_position_and_size(position: VectorU, size: VectorU) -> Self {
        Self { position, size }
    }

    /// Computes and returns the center of the rectangle.
    ///
    /// The center is calculated as the midpoint of the rectangle's width and height from its bottom-left corner.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = RectU::new(10, 20, 30, 40);
    /// let center = rect.center();
    /// assert_eq!(center, VectorU::new(25, 40));
    /// ```
    ///
    /// # Note
    ///
    /// This method performs integer division, which means that if the size of the rectangle is odd, the center coordinates may not be exact integers.
    pub fn center(self) -> VectorU {
        VectorU::new(
            self.position.x + self.size.x / 2,
            self.position.y + self.size.y / 2,
        )
    }

    /// Returns a new `RectU` instance with the specified offset applied to its position.
    ///
    /// The new rectangle will have the same size but its position will be adjusted by the given `offset`.
    ///
    /// # Arguments
    ///
    /// - `offset`: The `VectorU` to add to the rectangle's bottom-left position.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = RectU::new(10, 20, 30, 40);
    /// let offset = VectorU::new(5, 5);
    /// let new_rect = rect.with_offset(offset);
    /// assert_eq!(new_rect.position, VectorU::new(15, 25));
    /// ```
    pub fn with_offset(self, offset: VectorU) -> RectU {
        RectU::with_position_and_size(self.position + offset, self.size)
    }
}

impl From<(u32, u32, u32, u32)> for RectU {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self {
            position: (value.0, value.1).into(),
            size: (value.2, value.3).into(),
        }
    }
}


/// A 2D rectangle with integer coordinates for position and unsigned integer dimensions.
///
/// This struct represents a rectangle in a 2D space where the position is specified
/// using `VectorI` for integer coordinates, and the size (width and height) is represented
/// using `VectorU` for unsigned integer dimensions.
///
/// # Fields
///
/// - `position`: The bottom-left corner of the rectangle as a `VectorI`.
/// - `size`: The width and height of the rectangle as a `VectorU`.
///
/// # Examples
///
/// ```
/// use int_math::prelude::*;
///
/// // Create a rectangle with bottom-left corner at (10, 20) and size (30, 40)
/// let rect = RectI {
///     position: VectorI::new(10, 20),
///     size: VectorU::new(30, 40),
/// };
///
/// assert_eq!(rect.position, VectorI::new(10, 20));
/// assert_eq!(rect.size, VectorU::new(30, 40));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RectI {
    pub position: VectorI,
    pub size: VectorU,
}

impl RectI {
    /// Creates a new `RectI` instance with the specified position and size.
    ///
    /// # Arguments
    ///
    /// - `x`: The x-coordinate of the bottom-left corner of the rectangle.
    /// - `y`: The y-coordinate of the bottom-left corner of the rectangle.
    /// - `width`: The width of the rectangle.
    /// - `height`: The height of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = RectI::new(10, 20, 30, 40);
    /// assert_eq!(rect.position, VectorI::new(10, 20));
    /// assert_eq!(rect.size, VectorU::new(30, 40));
    /// ```
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: VectorI { x, y },
            size: VectorU {
                x: width,
                y: height,
            },
        }
    }

    /// Creates a new `RectI` instance with the specified position and size.
    ///
    /// # Arguments
    ///
    /// - `position`: The bottom-left corner of the rectangle as a `VectorI`.
    /// - `size`: The width and height of the rectangle as a `VectorU`.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let position = VectorI::new(10, 20);
    /// let size = VectorU::new(30, 40);
    /// let rect = RectI::with_position_and_size(position, size);
    /// assert_eq!(rect.position, position);
    /// assert_eq!(rect.size, size);
    /// ```
    pub fn with_position_and_size(position: VectorI, size: VectorU) -> Self {
        Self { position, size }
    }

    /// Computes and returns the center of the rectangle.
    ///
    /// The center is calculated as the midpoint of the rectangle's width and height from its bottom-left corner.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = RectI::new(10, 20, 30, 40);
    /// let center = rect.center();
    /// assert_eq!(center, VectorI::new(25, 40));
    /// ```
    ///
    /// # Note
    ///
    /// This method performs integer division, which means that if the size of the rectangle is odd, the center coordinates may not be exact integers.
    pub fn center(self) -> VectorI {
        VectorI::new(
            self.position.x + self.size.x as i32 / 2,
            self.position.y + self.size.y as i32 / 2,
        )
    }


    /// Returns a new `RectI` instance with the specified offset applied to its position.
    ///
    /// The new rectangle will have the same size but its position will be adjusted by the given `offset`.
    ///
    /// # Arguments
    ///
    /// - `offset`: The `VectorI` to add to the rectangle's position.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = RectI::new(10, 20, 30, 40);
    /// let offset = VectorI::new(5, -5);
    /// let new_rect = rect.with_offset(offset);
    /// assert_eq!(new_rect.position, VectorI::new(15, 15));
    /// ```
    pub fn with_offset(self, offset: VectorI) -> RectI {
        RectI::with_position_and_size(self.position + offset, self.size)
    }
}

impl From<(i32, i32, u32, u32)> for RectI {
    fn from(value: (i32, i32, u32, u32)) -> Self {
        Self {
            position: (value.0, value.1).into(),
            size: (value.2, value.3).into(),
        }
    }
}
