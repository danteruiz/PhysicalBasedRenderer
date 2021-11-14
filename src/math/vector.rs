// vector.rs
//
// Created on 2021/11/09 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

struct Vector<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    fn as_ptr(&self) -> *const T {
        &self.data[0]
    }
}

struct Point<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Point<T, N> {
    fn as_ptr(&self) -> *const T {
        &self.data[0]
    }
}

// type Vec2 = Vector<f32, 2>;
//
// type Point2 = Point<f32, 2>;
// type Point3 = Point<f32, 3>;
