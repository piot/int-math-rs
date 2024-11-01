/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/int-math-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use int_math::{URect, UVec2};

#[test]
fn check_size() {
    let result = URect::new(2, 2, 10, 16);
    assert_eq!(result.size.y, 16);
}

#[test]
fn check_vecu_mul() {
    let result = UVec2::new(2, 3) * 2;
    assert_eq!(result.x, 4);
    assert_eq!(result.y, 6);
}

#[test]
fn check_vecu_div() {
    let result = UVec2::new(5, 9) / 3;
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 3);
}

#[test]
fn check_vecu_add() {
    let result = UVec2::new(5, 9) + UVec2::new(7, 11);
    assert_eq!(result.x, 12);
    assert_eq!(result.y, 20);
}

#[test]
#[should_panic]
fn check_vecu_sub_overflow() {
    let result = UVec2::new(5, 9) - UVec2::new(7, 11);
    assert_eq!(result.x, 12);
    assert_eq!(result.y, 20);
}

#[test]
fn check_vecu_sub() {
    let result = UVec2::new(5, 9) - UVec2::new(4, 1);
    assert_eq!(result.x, 1);
    assert_eq!(result.y, 8);
}
