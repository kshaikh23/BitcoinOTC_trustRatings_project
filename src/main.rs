use std::fs::File;
use std::io::BufRead;
extern crate chrono;
use chrono::{Datelike, DateTime};
#[cfg(test)]
mod tests;
use plotters::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<(i32, i32, i32, f64)> = read_file("bitcoinOTC_trust_data.csv");

    let max_time: f64 = data[data.len()-1].3;
    let min_time: f64 = data[0].3;
    println!("Length of time dataset covers: {} years", (max_time - min_time)/31_536_000.0);

    let times: Vec<f64> = col_to_vec(&data, 3).get_flt_vec().unwrap();
    let times_month_year: Vec<(u32, i32)> = times.iter().map(|&seconds| epoch_to_date(seconds)).collect();
    let ratings: Vec<i32> = col_to_vec(&data, 2).get_int_vec().unwrap();
    println!("{:?} to {:?}", times_month_year[0], times_month_year[times_month_year.len() - 1]);

    // Makes a new times vector where the values are the months since the first month in the data
    let start_year: i32 = times_month_year[0].1;
    let start_month: u32 = times_month_year[0].0;
    let times_months_after_start: Vec<usize> = times_month_year.iter().map(|&(month, year)| {
        let years = year - start_year;
        let months = month as i32 - start_month as i32;
        return (years*12 + months) as usize 
    }).collect();

    // Create vectors for ratings over time plot
    // Mean rating for each month in dataset
    let mut ratings_by_month: Vec<f64> = Vec::new();
    // Numbering each month in dataset
    let mut unique_times_months_after_start: Vec<usize> = Vec::new();

    // Filling vectors for ratings over time plot
    let mut current_time = times_months_after_start[0];
    let mut current_ratings: Vec<f64> = Vec::new();
    for (&time, &rating) in times_months_after_start.iter().zip(ratings.iter()) {
        if time != current_time{
            // Calculate mean
            let mean_rating = current_ratings.iter().sum::<f64>() / current_ratings.len() as f64;
            ratings_by_month.push(mean_rating);
            unique_times_months_after_start.push(current_time);

            // Reset
            current_ratings.clear();
            current_time = time; 
        }
        current_ratings.push(rating as f64);
    }
    // For the last month
    let mean_rating = current_ratings.iter().sum::<f64>() / current_ratings.len() as f64;
    ratings_by_month.push(mean_rating);
    unique_times_months_after_start.push(current_time);

    // To ignore the resulting value that must be returned due to creating the graph
    let _ = time_ratings_plot(unique_times_months_after_start, ratings_by_month, start_month, start_year);

    // Creates filtered data with only trust ratings over 7
    let strong_ratings_data: Vec<(i32, i32, i32, f64)> = strong_ratings_only(&data);

    println!("Number of trust ratings over 7: {}", strong_ratings_data.len());

    // Must have main function return something due to creating the graph
    return Ok(())
}

// To read data from a csv file
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

pub fn time_ratings_plot(x: Vec<usize>, y: Vec<f64>, start_month: u32, start_year: i32) -> Result<(), Box<dyn std::error::Error>> {
    // Create file for graph to be displayed
    let root = BitMapBackend::new("ratingsOverTime_plot.png", (640, 480)).into_drawing_area();

    // Create chart to plot on
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Trust Ratings Over Time", ("sans-serif", 40))
        .x_label_area_size(35).y_label_area_size(40)
        .build_cartesian_2d(0..*x.last().unwrap(), -10.0..10.0)?;
    // Calculate ticks for the start of each year
    let year_ticks: Vec<usize> = x.iter().filter(|&&months| (months + start_month as usize - 1) % 12 == 0).cloned().collect();
    chart.configure_mesh().x_labels(x.len()).x_label_formatter(&|x| {
        let year_index = (start_year + ((*x as i32 + start_month as i32 - 1) / 12)) - start_year;
        if year_ticks.contains(&(*x as usize)) {
            return format!("{}", start_year + year_index)
        } else {
            return "".to_string()
        }
    }).draw()?;

    // Plotting
    chart.draw_series(LineSeries::new(x.iter().zip(y.iter()).map(|(&xi, &yi)| (xi,yi)), &RED))?;

    // Finalize graph
    root.present()?;

    // Print observations from plot
    println!("\nTrust Ratings Over Time plot observations: \nThe average trust rating dipped most significantly in August 2013 to about -2.3.");
    println!("It was also negative in December 2013 and December 2015.");
    println!("The highest the average trust rating has been was the first month on the dataset, November 2010.\n");

    // Must return something due to creating the graph
    return Ok(())
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

// Ignoring direction and not doing strongly CC as I don't care about direction to find communities
// Uses trees to track each connected component
struct ConnectedComponents {
    // Holds parent of each node, root nodes are where index and value are the same
    parent: Vec<usize>,
    // Measure to help keep each tree as flat as possible
    rank: Vec<usize>,
    // Stores the amount of nodes in each component for each root node
    size: Vec<usize>,
}

impl ConnectedComponents{
    // Takes n which is the amount of nodes in a dataset
    fn new(n: usize) -> Self {
        ConnectedComponents {
            parent: (0..n).collect(), 
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    // To find the root node of a node, recursively calls and makes every node along the way point to the root node, reducing the height of the tree
    fn find_root(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find_root(self.parent[u]);
        }
        return self.parent[u]; 
    }

    // To merge two components that connect between the nodes u and v
    fn merge(&mut self, u: usize, v: usize) {
        let root_u = self.find_root(u);
        let root_v = self.find_root(v); 
        if root_u != root_v {
            if self.rank[root_u] > self.rank[root_v] {
                self.parent[root_v] = root_u;
                self.size[root_u] += self.size[root_v];
            } else {
                self.parent[root_u] = root_v;
                self.size[root_v] += self.size[root_u];
                if self.rank[root_u] == self.rank[root_v] {
                    self.rank[root_v] += 1;
                } 
            }
        }
    }
}

pub fn node_count(data: &Vec<(i32, i32, i32, f64)>) -> usize {
    let mut nodes = HashSet::new();
    for &(u, v, _, _) in data {
        nodes.insert(u);
        nodes.insert(v);
    }
    return nodes.len()
}
