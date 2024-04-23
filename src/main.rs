use std::fs::File;
use std::io::BufRead;

fn main() {
    let data = read_file("bitcoinOTC_trust_data.csv");

    for line in data {
        println!("{}, {}, {}, {}", line.0, line.1, line.2, line.3);
    }
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