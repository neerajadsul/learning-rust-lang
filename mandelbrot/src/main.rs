use num::Complex;
use std::str::FromStr;
use std::env;
use crossbeam;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!("Usage: {} FILE PIXELS TOP-LEFT BOTTOM-RIGHT NUM_THREADS ", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20 8", args[0]);
        std::process::exit(1);
    }

    let fname = &args[1];
    let bounds = parse_pair(&args[2], 'x')
        .expect("Error parxing image dimensions");
    let top_left = parse_complex(&args[3])
        .expect("Error parsing top left corner point.");
    let bot_right = parse_complex(&args[4])
        .expect("Error parsing bottom right corner point");
    let num = &args[5];
    let num_threads:usize = match num.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error: last argument is not integer");
            return;
        },
    };

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = num_threads;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [u8]> =
            pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_top_left =
                    pixel_to_point(bounds, (0, top), top_left, bot_right);
                let band_bot_right =
                    pixel_to_point(bounds, (bounds.0, top + height), top_left, bot_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_top_left, band_bot_right);
                });
            }
        }).unwrap();
    }

    render(&mut pixels, bounds, top_left, bot_right);

    write_image(fname, &pixels, bounds)
        .expect("Error writing PNG File");

}

/// Parse the string `s` as a coordinate pair, like 400x600 or 1.0,0.5
/// `s` should have the form <left><sep><right>
/// If `s` has the proper form, return `Some(<x, y>), if it does not parse
/// correctly then return `None`
fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    match s.find(sep) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(left), Ok(right)) => Some((left, right)),
                _ => None
            }
        }
    }
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("20,10", ','), Some((20, 10)));
    assert_eq!(parse_pair::<i32>("20,10xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}
#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0425"), Some(Complex {re: 1.25, im: -0.0425}));
    assert_eq!(parse_complex(",-0.01"), None);
}
/// Check if `c` is in the Mandelbrot set, using at most `limit` iterations.
///
/// If `c` is not a member, return Some(i), where `i` is the number of iterations it took
/// for `c` to leave the circle of radius 2 centered at origin.
/// If `c` seems to be a member, return None. (more precisely if we can not prove
/// that it is not a member within the iteration limit.)
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re:0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Given the row and column of a pixel in the output image,
/// return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in the image.
/// The `top_left` and `bot_right` parameters are points on the complex plane
/// designating the area our image covers
fn pixel_to_point(bounds: (usize, usize),
                pixel: (usize, usize),
                top_left: Complex<f64>,
                bot_right: Complex<f64>)
    -> Complex<f64>
{
    let (width, height) = (bot_right.re - top_left.re,
                            top_left.im - bot_right.im);
    Complex {
        re: top_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: top_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // pixel.1 increases as we go down but the
        // imaginary component increases as we go up
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100), (25, 75),
                            Complex {re: -1.0, im: 1.0},
                            Complex {re: 1.0, im: -1.0}),
                    Complex {re: -0.5, im: -0.5});
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels
///
/// The `bounds` given the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `top_left` and `bot_right`
/// arguments specify the points on the complex plane corresponding to the
/// top-left and bottom-right corners of the pixel buffers.
fn render(pixels: &mut [u8],
        bounds: (usize, usize),
        top_left: Complex<f64>,
        bot_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), top_left, bot_right);
            pixels[row * bounds.0 + col] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}