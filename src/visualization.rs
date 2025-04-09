use plotters::prelude::*;
use std::error::Error;

/// Generates a simple line chart comparing the success rates of different error correction codes
pub fn plot_success_rates(
    bit_flip_success: f64,
    phase_flip_success: f64,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    // Create a drawing area
    let root = SVGBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Error Correction Success Rates", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..3, 0.0..100.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_desc("Success Rate (%)")
        .draw()?;

    // Plot the data points
    chart.draw_series(LineSeries::new(
        vec![(1, bit_flip_success * 100.0), (2, phase_flip_success * 100.0)],
        &BLUE,
    ))?
        .label("Success Rates")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Add text labels
    root.draw(&Text::new(
        "Bit Flip Code",
        (180, 500),
        ("sans-serif", 20),
    ))?;

    root.draw(&Text::new(
        "Phase Flip Code",
        (500, 500),
        ("sans-serif", 20),
    ))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Chart has been saved to {}", output_file);

    Ok(())
}

/// Generates a line chart showing how error rates affect success rates
pub fn plot_error_vs_success(
    error_rates: &[f64],
    bit_flip_success_rates: &[f64],
    phase_flip_success_rates: &[f64],
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    // Create a drawing area
    let root = SVGBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_success = bit_flip_success_rates
        .iter()
        .chain(phase_flip_success_rates.iter())
        .fold(0.0, |max, &val| if val > max { val } else { max }) * 100.0;

    let mut chart = ChartBuilder::on(&root)
        .caption("Error Rate vs. Success Rate", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0.0..(*error_rates.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() * 1.1),
            0.0..(max_success * 1.1),
        )?;

    chart
        .configure_mesh()
        .x_desc("Error Rate")
        .y_desc("Success Rate (%)")
        .draw()?;

    // Plot bit flip success rates
    chart.draw_series(LineSeries::new(
        error_rates.iter().zip(bit_flip_success_rates.iter()).map(|(&x, &y)| (x, y * 100.0)),
        &RED,
    ))?
        .label("Bit Flip Code")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Plot phase flip success rates
    chart.draw_series(LineSeries::new(
        error_rates.iter().zip(phase_flip_success_rates.iter()).map(|(&x, &y)| (x, y * 100.0)),
        &BLUE,
    ))?
        .label("Phase Flip Code")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Chart has been saved to {}", output_file);

    Ok(())
}