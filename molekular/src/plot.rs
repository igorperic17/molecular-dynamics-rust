use plotters::prelude::*;
use plotters::data::fitting_range;

pub fn plot(vec: &Vec<f64>, file: &str) {

    let root_area = BitMapBackend::new(file, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let x_range: Vec<f64> = (0..vec.len()).map(|x| x as f64).collect();
    let y_range = fitting_range(vec);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .caption("Electron position X", ("sans-serif", 40.0))
        .build_cartesian_2d(0.0..vec.len() as f64, y_range)
        .unwrap();


    let ranges = x_range.into_iter().zip(vec.iter().map(|x| *x));
    ctx.draw_series(
        AreaSeries::new(
            ranges,
            0.0, 
            &RED.mix(0.2))
            .border_style(&RED))
            .unwrap();
}