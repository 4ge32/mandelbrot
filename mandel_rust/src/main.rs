extern crate num;

use num::complex::Complex;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;

fn mandelbrot(c: Complex<f64>, k: f64, lp: i32) -> i32 {
    let mut z: Complex<f64> = Complex::new(0.0, 0.0);
    for i in 0..lp {
        z = z * z + c;
        if z.norm_sqr() > k {
            return i
        }
    }
    lp
}

fn main() -> std::io::Result<()> {
    let x_min = -2.0;
    let x_max = 2.0;
    let y_min = -2.0;
    let y_max = 2.0;
    let lp = 20;
    let step = 0.01;
    let k = 4.0;

    let mut file = BufWriter::new(File::create("mandel.png.data")?);

    let mut x = x_min;
    while x < x_max {
        let mut y = y_min;
        while y < y_max {
            let c: Complex<f64> = Complex::new(x, y);
            let n = mandelbrot(c, k, lp);
            write!(file, "{:.10}\t{:.10}\t{}\n", c.re, c.im, n)?;
            y += step;
        }
        write!(file, "\n")?;
        x += step;
    }
    println!("Hello, world!");
    Ok(())
}
