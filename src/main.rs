use std::fs::File;
use std::io::BufRead;
#[cfg(test)]
mod tests;

fn main() {
    let data: Vec<(i32, i32, i32, f64)> = read_file("bitcoinOTC_trust_data.csv");

    let max_time: f64 = data[data.len()-1].3;
    let min_time: f64 = data[0].3;
    println!("Length of time dataset covers: {} years", (max_time - min_time)/31_536_000.0);
    println!("{} to {}", 1970.0 + min_time/31_536_000.0, 1970.0 + max_time/31_536_000.0);

    let times_enum: VecType = col_to_vec(&data, 3);
    let times: Vec<f64> = times_enum.get_flt_vec().unwrap();
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

impl VecType {
    pub fn get_int_vec(self) -> Option<Vec<i32>> {
        if let VecType::IntVec(vec) = self {
            return Some(vec)
        } else {
            return None
        }
    }

    pub fn get_flt_vec(self) -> Option<Vec<f64>> {
        if let VecType::FltVec(vec) = self {
            return Some(vec)
        } else {
            return None
        }
    }
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