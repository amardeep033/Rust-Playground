use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area 800x600 pixels
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Set up the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Sample Plot: y = x^2", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..100, 0..10000)?;

    chart.configure_mesh().draw()?;

    // Draw the curve
    chart.draw_series(
        (0..100).map(|x| (x, x * x))
            .map(|(x, y)| Circle::new((x, y), 3, RED.filled()))
    )?;

    println!("Plot saved to output.png");



        let data = vec![1.,2.,2.,3.,3.,3.,4.,4.,4.,4.];

    Chart::new(80, 20, 0., 5.)
        .histogram(&data)
        .display();
    Ok(())
}
