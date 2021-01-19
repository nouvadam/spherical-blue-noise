use plotters::prelude::*;
use spherical_blue_noise::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sphericalbluenoise = BlueNoiseSphere::new(4096, &mut rand::thread_rng());

    let area = SVGBackend::new("examples/plot.svg", (1024, 760)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (-1.0..1.0).step(0.1);
    let z_axis = (-1.0..1.0).step(0.1);

    let mut chart =
        ChartBuilder::on(&area).build_cartesian_3d(x_axis.clone(), -1.0..1.0, z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.0;
        pb.pitch = 0.0;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    let points_to_draw: Vec<(f64, f64, f64)> = sphericalbluenoise
        .clone()
        .into_iter()
        .filter(|p| p.2 > 0.0)
        .map(|p| (p.0 as f64, p.1 as f64, p.2 as f64))
        .collect();

    chart.draw_series(PointSeries::of_element(
        points_to_draw,
        2,
        &BLACK,
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
        },
    ))?;

    area.present()?;

    chart.configure_series_labels().draw()?;
    Ok(())
}
