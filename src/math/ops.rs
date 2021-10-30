// ops.rs
//
// Created on 2021/10/18 by Dante Ruiz
// Copyright 2021 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(&self, rhs: &Rhs) -> f32;
}

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(&self, rhs: &Rhs) -> Self::Output;
}

pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub trait Transpose {
    fn transpose(&self) -> Self;
}

pub trait Inverse {
    fn inverse(&self) -> Self;
}

pub trait Determinant {
    fn determinant(&self) -> f32;
}
