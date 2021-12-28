use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64,
}

fn main() {
    let port_number = "5555";
    let host_addr = "localhost";

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Running web-server on http://localhost:{}", port_number);
    server
        .bind(format!("{}:{}",host_addr, port_number)).expect("error binding server to address")
        .run().expect("error running the server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title> GCD Calculator </title>
                <form action="gcd" method="post">
                <input type="text" name="m">
                <input type="text" name="n">
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("GCD of zero is meaningless");
    }

    let response = format!(
        "The GCD of numbers {} and {} is <b>{}</b>",
        form.m, form.n, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!( n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t
        }
        m = m % n;
    }
    n
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 23, 19 * 23 * 37), 23);
    assert_eq!(gcd(108, 210), 6);
}
