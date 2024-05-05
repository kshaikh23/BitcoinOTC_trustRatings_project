mod data_manipulation;
use data_manipulation::data_manipulation::{read_file, col_to_vec, epoch_to_date, strong_ratings_only};
mod connected_components;
use connected_components::connected_components::{components_and_sizes};
mod visuals;
use visuals::visuals::{time_ratings_plot, ratings_distribution_bargraph};
#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<(i32, i32, i32, f64)> = read_file("bitcoinOTC_trust_data.csv");

    let max_time: f64 = data[data.len()-1].3;
    let min_time: f64 = data[0].3;
    println!("\nLength of time dataset covers: {} years", (max_time - min_time)/31_536_000.0);

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
    println!("Full data:\nConnected Components: {}\nSize of each component: {:?}\nTotal amount of nodes: {}\n", full_data_components, full_data_component_sizes, full_data_component_sizes.iter().sum::<usize>()); 

    let (strong_ratings_data_components, strong_ratings_data_component_sizes) = components_and_sizes(&strong_ratings_data);
    println!("Strong ratings only data:\nConnected Components: {}\nSize of each component: {:?}\nTotal amount of nodes: {}\n", strong_ratings_data_components, strong_ratings_data_component_sizes, strong_ratings_data_component_sizes.iter().sum::<usize>()); 

    // Must have main function return something due to creating the graph
    return Ok(())
}