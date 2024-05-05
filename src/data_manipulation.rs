pub mod data_manipulation {
    use std::fs::File;
    use std::io::BufRead;
    extern crate chrono;
    use chrono::{Datelike, DateTime};

    // To read data from a csv file
    pub fn read_file(path: &str) -> Vec<(i32, i32, i32, f64)> {
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

    // Enum made so either a vector of i32 or f64 can be returned
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum VecType {
        IntVec(Vec<i32>),
        FltVec(Vec<f64>),
    }

    // Functions to unwrap VecType enum
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

    // Takes a column of the data and turns it into a vector
    pub fn col_to_vec(data: &Vec<(i32, i32, i32, f64)>, col: i32) -> VecType {
        match col {
            0 => VecType::IntVec(data.into_iter().map(|line| line.0).collect()),
           1 => VecType::IntVec(data.into_iter().map(|line| line.1).collect()),
           2 => VecType::IntVec(data.into_iter().map(|line| line.2).collect()),
           3 => VecType::FltVec(data.into_iter().map(|line| line.3).collect()),
           _ => panic!("Column index is out of bounds."),
        }   
    }

    // Converts seconds since epoch to a tuple containing the month and year
    pub fn epoch_to_date(seconds: f64) -> (u32, i32) {
        let datetime = DateTime::from_timestamp(seconds as i64, 0);
        return (datetime.expect("Error in epoch_to_date month").month(), datetime.expect("Error in epoch_to_date year").year())
    }

    // Returns filtered data where all edges have a trust rating above 7
    pub fn strong_ratings_only(data: &Vec<(i32, i32, i32, f64)>) -> Vec<(i32, i32, i32, f64)> {
        let mut new_data: Vec<(i32, i32, i32, f64)> = Vec::new();
        for line in data {
           if line.2 > 7 {
               new_data.push(*line);
          }
        }
        return new_data;
    }
}