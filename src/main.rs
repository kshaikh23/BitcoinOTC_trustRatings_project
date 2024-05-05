#[cfg(test)]
mod tests;
mod data_manipulation;
use data_manipulation::data_manipulation::{read_file, col_to_vec, epoch_to_date, strong_ratings_only};
mod connected_components;
use connected_components::connected_components::{components_and_sizes};
use plotters::prelude::*;
use std::collections::HashMap;

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

    // To ignore the resulting values that must be returned due to creating the graphs
    let _ = time_ratings_plot(unique_times_months_after_start, ratings_by_month, start_month, start_year);
    let _ = ratings_distribution_bargraph(&ratings);

    // Creates filtered data with only trust ratings over 7
    let strong_ratings_data: Vec<(i32, i32, i32, f64)> = strong_ratings_only(&data);

    println!("Number of trust ratings over 7: {} ({:.2}%)\n", strong_ratings_data.len(), (strong_ratings_data.len() as f64 / data.len() as f64)*100.0);

    let (full_data_components, full_data_component_sizes) = components_and_sizes(&data);
    println!("Full data:\nConnected Components: {}\nSize of each component: {:?}\n", full_data_components, full_data_component_sizes); 

    let (strong_ratings_data_components, strong_ratings_data_component_sizes) = components_and_sizes(&strong_ratings_data);
    println!("Strong ratings only data:\nConnected Components: {}\nSize of each component: {:?}\n", strong_ratings_data_components, strong_ratings_data_component_sizes); 

    // Must have main function return something due to creating the graph
    return Ok(())
}

// Makes a plot that shows how the mean trust rating changes from month to month
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

    // Finalize plot
    root.present()?;

    // Print observations from plot
    println!("\nTrust Ratings Over Time plot observations: \nThe average trust rating dipped most significantly in August 2013 to about -2.3.");
    println!("It was also negative in December 2013 and December 2015.");
    println!("The highest the average trust rating has been was the first month on the dataset, November 2010.\n");

    // Must return something due to creating the plot
    return Ok(())
}

// Makes a bar graph that shows the distribution of trust ratings
pub fn ratings_distribution_bargraph(ratings: &Vec<i32>) -> Result<(), Box<dyn std::error::Error>> {
    // Count the number of occurences of each rating
    let mut counts = HashMap::new();
    for &rating in ratings {
        *counts.entry(rating).or_insert(0) += 1;
    }

    // Creates area to make bar graph on
    let root = BitMapBackend::new("ratingsDistribution_barGraph.png", (640, 480)).into_drawing_area();

    let max_count = counts.values().max().unwrap_or(&0);

    // Create chart to draw bar graph on
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Trust Ratings Distribution", ("sans-serif", 40))
        .x_label_area_size(35).y_label_area_size(40)
        .build_cartesian_2d(-11..11, 0..*max_count + 1)?;
    chart.configure_mesh().x_labels(22).y_labels(10).draw()?;

    // Draw bars
    chart.draw_series(
        (-10..=10).map(|x| {
            let count = counts.get(&x).unwrap_or(&0);
            let bar = Rectangle::new([(x, 0), (x+1, *count)], BLUE.filled(),);
            return bar
        })
    )?;

    // Finalize bar graph
    root.present()?;

    // Must return something due to creating the bar graph
    return Ok(())
}