use std::fs::File;
use std::io::BufRead;
#[cfg(test)]
mod tests;

fn main() {
    let data = read_file("bitcoinOTC_trust_data.csv");
}

fn read_file(path: &str) -> Vec<(i32, i32, i32, f64)> {
    let mut result: Vec<(i32, i32, i32, f64)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let reader = std::io::BufReader::new(file).lines().skip(1);
    for line in reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let a = v[0].parse::<i32>().unwrap();
        let b = v[1].parse::<i32>().unwrap();
        let c = v[2].parse::<i32>().unwrap();
        let d = v[3].parse::<f64>().unwrap();
        result.push((a, b, c, d));
    }
    return result
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum VecType {
    IntVec(Vec<i32>),
    FltVec(Vec<f64>),
}

pub fn col_to_vec(data: &Vec<(i32, i32, i32, f64)>, col: i32) -> VecType {
    match col {
        0 => VecType::IntVec(data.into_iter().map(|line| line.0).collect()),
        1 => VecType::IntVec(data.into_iter().map(|line| line.1).collect()),
        2 => VecType::IntVec(data.into_iter().map(|line| line.2).collect()),
        3 => VecType::FltVec(data.into_iter().map(|line| line.3).collect()),
        _ => panic!("Column index is out of bounds."),
    }
}