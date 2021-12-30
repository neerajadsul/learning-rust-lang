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
