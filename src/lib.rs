/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/int-math-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
extern crate core;
pub mod prelude;

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
/// let position = UVec2::new(5, 10);
/// println!("{:?}", position); // Output: UVec2 { x: 5, y: 10 }
/// ```
///
/// # Fields
///
/// - `x`: The bottom-left x-coordinate (horizontal position) of the vector.
/// - `y`: The bottom-left y-coordinate (vertical position) of the vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UVec2 {
    pub x: u16,
    pub y: u16,
}

impl UVec2 {
    /// Creates a new `UVec2` instance with the specified coordinates.
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
    /// let vector = UVec2::new(5, 10);
    /// assert_eq!(vector.x, 5);
    /// assert_eq!(vector.y, 10);
    /// ```
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for UVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Self> for UVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<u16> for UVec2 {
    type Output = Self;

    fn div(self, rhs: u16) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<u16> for UVec2 {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(u16, u16)> for UVec2 {
    fn from(value: (u16, u16)) -> Self {
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
/// let position = Vec2::new(-5, 10);
/// println!("{:?}", position); // Output: Vec2 { x: -5, y: 10 }
/// ```
///
/// # Fields
///
/// - `x`: The bottom-left x-coordinate (horizontal position) of the vector.
/// - `y`: The bottom-left y-coordinate (vertical position) of the vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i16,
    pub y: i16,
}

impl Vec2 {
    /// Creates a new `Vec2` instance with the specified coordinates.
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
    /// let vector = Vec2::new(-5, 10);
    /// assert_eq!(vector.x, -5);
    /// assert_eq!(vector.y, 10);
    /// ```
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i16> for Vec2 {
    type Output = Self;

    fn div(self, rhs: i16) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<i16> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(i16, i16)> for Vec2 {
    fn from(value: (i16, i16)) -> Self {
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
/// let position = Vec3::new(-5, 10, 0);
/// println!("{:?}", position); // Output: Vec3 { x: -5, y: 10 }
/// ```
///
/// # Fields
///
/// - `x`: The bottom-left x-coordinate (horizontal position) of the vector.
/// - `y`: The bottom-left y-coordinate (vertical position) of the vector.
/// - `z`: The z-coordinate (depth position) of the vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl From<Vec2> for Vec3 {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: 0,
        }
    }
}

impl Vec3 {
    /// Creates a new `Vec2` instance with the specified coordinates.
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
    /// let vector = Vec3::new(-5, 10, -99);
    /// assert_eq!(vector.x, -5);
    /// assert_eq!(vector.y, 10);
    /// assert_eq!(vector.z, -99);
    /// ```
    #[must_use]
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<i16> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i16) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<i16> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl From<(i16, i16, i16)> for Vec3 {
    fn from(value: (i16, i16, i16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
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
/// let rect = URect::new(10, 20, 30, 40);
/// println!("{:?}", rect); // Output: URect { position: UVec2 { x: 10, y: 20 }, size: UVec2 { x: 30, y: 40 } }
/// ```
///
/// # Fields
///
/// - `position`: The lower-left corner of the rectangle as a `UVec2`.
/// - `size`: The width and height of the rectangle as a `UVec2`.
#[derive(Debug, Clone, Copy)]
pub struct URect {
    pub position: UVec2,
    pub size: UVec2,
}

impl URect {
    /// Creates a new `URect` instance with the specified position and size.
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
    /// let rect = URect::new(10, 20, 30, 40);
    /// assert_eq!(rect.position.x, 10);
    /// assert_eq!(rect.position.y, 20);
    /// assert_eq!(rect.size.x, 30);
    /// assert_eq!(rect.size.y, 40);
    /// ```
    #[must_use]
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            position: UVec2 { x, y },
            size: UVec2 {
                x: width,
                y: height,
            },
        }
    }

    /// Creates a new `URect` instance with the specified position and size.
    ///
    /// # Arguments
    ///
    /// - `position`: The bottom-left corner of the rectangle as a `UVec2`.
    /// - `size`: The width and height of the rectangle as a `UVec2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let position = UVec2::new(10, 20);
    /// let size = UVec2::new(30, 40);
    /// let rect = URect::with_position_and_size(position, size);
    /// assert_eq!(rect.position, position);
    /// assert_eq!(rect.size, size);
    /// ```
    #[must_use]
    pub const fn with_position_and_size(position: UVec2, size: UVec2) -> Self {
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
    /// let rect = URect::new(10, 20, 30, 40);
    /// let center = rect.center();
    /// assert_eq!(center, UVec2::new(25, 40));
    /// ```
    ///
    /// # Note
    ///
    /// This method performs integer division, which means that if the size of the rectangle is odd, the center coordinates may not be exact integers.
    #[must_use]
    pub const fn center(self) -> UVec2 {
        UVec2::new(
            self.position.x + self.size.x / 2,
            self.position.y + self.size.y / 2,
        )
    }

    /// Returns a new `URect` instance with the specified offset applied to its position.
    ///
    /// The new rectangle will have the same size but its position will be adjusted by the given `offset`.
    ///
    /// # Arguments
    ///
    /// - `offset`: The `UVec2` to add to the rectangle's bottom-left position.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = URect::new(10, 20, 30, 40);
    /// let offset = UVec2::new(5, 5);
    /// let new_rect = rect.with_offset(offset);
    /// assert_eq!(new_rect.position, UVec2::new(15, 25));
    /// ```
    #[must_use]
    pub fn with_offset(self, offset: UVec2) -> Self {
        Self::with_position_and_size(self.position + offset, self.size)
    }
}

impl From<(u16, u16, u16, u16)> for URect {
    fn from(value: (u16, u16, u16, u16)) -> Self {
        Self {
            position: (value.0, value.1).into(),
            size: (value.2, value.3).into(),
        }
    }
}

/// A 2D rectangle with integer coordinates for position and unsigned integer dimensions.
///
/// This struct represents a rectangle in a 2D space where the position is specified
/// using `Vec2` for integer coordinates, and the size (width and height) is represented
/// using `UVec2` for unsigned integer dimensions.
///
/// # Fields
///
/// - `position`: The bottom-left corner of the rectangle as a `Vec2`.
/// - `size`: The width and height of the rectangle as a `UVec2`.
///
/// # Examples
///
/// ```
/// use int_math::prelude::*;
///
/// // Create a rectangle with bottom-left corner at (10, 20) and size (30, 40)
/// let rect = Rect {
///     position: Vec2::new(10, 20),
///     size: UVec2::new(30, 40),
/// };
///
/// assert_eq!(rect.position, Vec2::new(10, 20));
/// assert_eq!(rect.size, UVec2::new(30, 40));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub position: Vec2,
    pub size: UVec2,
}

impl Rect {
    /// Creates a new `Rect` instance with the specified position and size.
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
    /// let rect = Rect::new(10, 20, 30, 40);
    /// assert_eq!(rect.position, Vec2::new(10, 20));
    /// assert_eq!(rect.size, UVec2::new(30, 40));
    /// ```
    #[must_use]
    pub const fn new(x: i16, y: i16, width: u16, height: u16) -> Self {
        Self {
            position: Vec2 { x, y },
            size: UVec2 {
                x: width,
                y: height,
            },
        }
    }

    /// Creates a new `Rect` instance with the specified position and size.
    ///
    /// # Arguments
    ///
    /// - `position`: The bottom-left corner of the rectangle as a `Vec2`.
    /// - `size`: The width and height of the rectangle as a `UVec2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let position = Vec2::new(10, 20);
    /// let size = UVec2::new(30, 40);
    /// let rect = Rect::with_position_and_size(position, size);
    /// assert_eq!(rect.position, position);
    /// assert_eq!(rect.size, size);
    /// ```
    #[must_use]
    pub const fn with_position_and_size(position: Vec2, size: UVec2) -> Self {
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
    /// let rect = Rect::new(10, 20, 30, 40);
    /// let center = rect.center();
    /// assert_eq!(center, Vec2::new(25, 40));
    /// ```
    ///
    /// # Note
    ///
    /// This method performs integer division, which means that if the size of the rectangle is odd, the center coordinates may not be exact integers.
    #[must_use]
    pub const fn center(self) -> Vec2 {
        Vec2::new(
            self.position.x + self.size.x as i16 / 2,
            self.position.y + self.size.y as i16 / 2,
        )
    }

    /// Returns a new `Rect` instance with the specified offset applied to its position.
    ///
    /// The new rectangle will have the same size but its position will be adjusted by the given `offset`.
    ///
    /// # Arguments
    ///
    /// - `offset`: The `Vec2` to add to the rectangle's position.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::prelude::*;
    ///
    /// let rect = Rect::new(10, 20, 30, 40);
    /// let offset = Vec2::new(5, -5);
    /// let new_rect = rect.with_offset(offset);
    /// assert_eq!(new_rect.position, Vec2::new(15, 15));
    /// ```
    #[must_use]
    pub fn with_offset(self, offset: Vec2) -> Self {
        Self::with_position_and_size(self.position + offset, self.size)
    }
}

impl From<(i16, i16, u16, u16)> for Rect {
    fn from(value: (i16, i16, u16, u16)) -> Self {
        Self {
            position: (value.0, value.1).into(),
            size: (value.2, value.3).into(),
        }
    }
}
