pub mod visuals {
    use plotters::prelude::*;
    use std::collections::HashMap;

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
}