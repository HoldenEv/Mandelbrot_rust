use plotters::prelude::*;
use num_complex::Complex;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MAX_ITER: u32 = 100;

fn mandelbrot(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./test.svg";
    let root = SVGBackend::new(file_path, (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Mandelbrot Set", ("sans-serif", 40))
        .margin(10)
        .build_cartesian_2d(-2.0..1.0, -1.5..1.5)?;

    chart.configure_mesh().draw()?;

    let drawing_area = chart.plotting_area();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let re = (x as f64 / WIDTH as f64) * 3.0 - 2.0;
            let im = (y as f64 / HEIGHT as f64) * 3.0 - 1.5;

            let c = Complex::new(re, im);
            let iters = mandelbrot(c);

            let color = if iters == MAX_ITER {
                BLACK
            } else {
                RGBColor((iters * 5) as u8, (iters * 2) as u8, (iters * 3) as u8)
            };

            drawing_area.draw_pixel((re, im), &color)?;
        }
    }

    root.present()?;
    println!("Mandelbrot set rendered to {}", file_path);

    Ok(())
}
