use num::Complex;
use std::str::FromStr;

fn main() {
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