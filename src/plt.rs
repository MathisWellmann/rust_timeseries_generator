use plotters::prelude::*;

// prepare_vec returns a 2d vector suitable for plotting and also min, max values of input vector
fn prepare_vec(vals: Vec<f64>) -> (Vec<(f32, f64)>, f64, f64) {
    let mut out = vec![(0.0, 0.0); vals.len()];
    let mut min = vals[0];
    let mut max = vals[0];

    for i in 0..vals.len() {
        out[i] = (i as f32, vals[i]);
        if vals[i] > max {
            max = vals[i]
        } else if vals[i] < min {
            min = vals[i]
        }
    }
    return (out, min, max)
}

pub fn plt(vals: Vec<f64>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (vec2d, min, max) = prepare_vec(vals);

    // plot the resulting function
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(filename, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0f32..vec2d.len() as f32, min..max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(vec2d, &RED))?
        .label(filename);

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}