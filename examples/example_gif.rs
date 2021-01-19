use plotters::prelude::*;
use spherical_blue_noise::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let num_of_particles = 4096;
    let mut sphericalbluenoise = BlueNoiseSphere::new_raw(num_of_particles, &mut rng);

    let root = BitMapBackend::gif("examples/plot.gif", (1024, 760), 100)?.into_drawing_area();

    let mut max_vel = 0.03;

    for i in 0..=31 {
        root.fill(&WHITE)?;

        let x_axis = (-1.5..1.5).step(0.1);
        let z_axis = (-1.5..1.5).step(0.1);

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Iteration: {}", i), ("sans-serif", 50).into_font())
            .build_cartesian_3d(x_axis.clone(), -1.5..1.5, z_axis.clone())?;

        chart.with_projection(|mut pb| {
            pb.yaw = 0.0;
            pb.pitch = 0.0;
            pb.scale = 1.5;
            pb.into_matrix()
        });

        let points_to_draw: Vec<(f64, f64, f64)> = sphericalbluenoise
            .clone()
            .into_iter()
            .filter(|p| p.2 < 0.0)
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

        sphericalbluenoise = sphericalbluenoise.advance(max_vel);
        max_vel *= 0.8;

        root.present()?;
    }
    Ok(())
}
