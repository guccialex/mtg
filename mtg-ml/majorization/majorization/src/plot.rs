use plotters::prelude::*;



pub fn plot() {


    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();


    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32).unwrap();



    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();


    /*
    root.fill(&WHITE).unwrap();


    chart.configure_mesh().draw().unwrap();


    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        )).unwrap()
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        
    */


}
