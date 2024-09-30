#![forbid(unsafe_code)]

use core::panic;

pub fn flatten<const N: usize>(data: Vec<Box<[&mut i32; N]>>) -> Vec<&mut i32> {
    let mut result = Vec::with_capacity(N * N);

    for pointer in data {
        for num in *pointer {
            result.push(num);
        }
    }

    result
}

pub fn transform_to_fixed_arr<const N: usize>(data: &mut Vec<Vec<i32>>) -> Vec<Box<[&mut i32; N]>> {
    for vec in &*data {
        if vec.len() != N {
            panic!("Inner vectors are of different size")
        }
    }

    let mut result = Vec::with_capacity(N);
    for vec in data {
        let mut tmp_vec = Vec::with_capacity(N);
        for num in vec {
            tmp_vec.push(num);
        }
        result.push(Box::new(tmp_vec.try_into().unwrap()));
    }

    result
}