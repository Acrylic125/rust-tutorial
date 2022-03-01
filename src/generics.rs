use std::ops::*;

pub fn try_generics() {
    println!("{}", max_of_vec(vec![1, 2, 3]));
    println!("{}", max_of_vec(vec![1.0, 2.2, 3.3, 4.1, 5.3]));
}

pub fn max_of_vec<T: Copy + PartialOrd>(vec: Vec<T>) -> T {
    let mut highest = vec[0];
    for ele in vec {
        if ele > highest {
            highest = ele;
        }
    }
    highest
}

pub struct Vector<T: Add + Sub + Mul> {
    pub x: T,
    pub y: T,
}

impl<T: Add + Sub + Mul> Vector<T> {
    pub fn new(x: T, y: T) -> Vector<T> {
        Vector { x, y }
    }
}

impl Vector<f32> {
    pub fn normalize(&self) -> Vector<f32> {
        return Vector {
            x: self.x,
            y: self.y,
        };
    }
}
