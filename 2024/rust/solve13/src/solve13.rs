extern crate nalgebra as na;
use na::Matrix2;

use std::io::read_to_string;
use std::fs::File;

fn parse_eq(eq: &str) -> Vec<f64> {
    let eq_parts = eq.split(":").collect::<Vec<&str>>()[1].trim().split(",").collect::<Vec<&str>>();
    eq_parts.iter().map(|&x| x.split("+").collect::<Vec<&str>>()[1].trim().parse::<f64>().unwrap()).collect()
}

fn parse_price(price: &str) -> Vec<f64> {
    let price_parts = price.split(":").collect::<Vec<&str>>()[1].trim().split(",").collect::<Vec<&str>>();
    price_parts.iter().map(|&x| x.split("=").collect::<Vec<&str>>()[1].trim().parse::<f64>().unwrap()).collect()
}

fn parse_input(filename: &str) -> Vec<(Matrix2<f64>, [f64; 2])> {
    let input = read_to_string(File::open(filename).unwrap()).unwrap();
    let mut result = Vec::new();
    for batch in input.split("\n\n") {
        let lines: Vec<&str> = batch.lines().collect();
        let A = parse_eq(lines[0]);
        let B = parse_eq(lines[1]);
        let price = parse_price(lines[2]);
        result.push((Matrix2::new(A[0], B[0], A[1], B[1]), [price[0], price[1]]));
    }
    result
}

fn cramer(matrix: Matrix2<f64>, price: [f64; 2]) -> Option<[f64; 2]> {
    let det = matrix.determinant();
    let mut result = [0.0, 0.0];
    for i in 0..2 {
        let mut new_matrix = matrix.clone();
        new_matrix[(0, i)] = price[0];
        new_matrix[(1, i)] = price[1];
        result[i] = new_matrix.determinant() / det;
    }

    // Check if result is close enough to integer
    if result.iter().all(|&x| (x - x.round()).abs() < 0.0001) {
        Some(result)
    } else {
        None
    }
}

fn main() {
    let equations = parse_input("input13");

    let mut sum = 0; 
    for eq in equations.iter() {
        if let Some(result) = cramer(eq.0, eq.1) {
            sum += result[0] as i32 * 3 + result[1] as i32;
        }
    }
    println!("Part 1: {}", sum);

    let mut sum2 = 0;
    for eq in equations.iter() {
        let new_price = [eq.1[0] + 10000000000000.0, eq.1[1] + 10000000000000.0];
        if let Some(result) = cramer(eq.0, new_price) {
            sum2 += result[0] as i64 * 3 + result[1] as i64;
        }
    }
    println!("Part 2: {}", sum2);
}