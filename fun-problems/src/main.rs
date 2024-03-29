
fn main() {

    for args in 1..20 {
        println!("{}", fizz_buzz(args));
    }

}


fn fizz_buzz(args: i32) -> String {
    let mut msg = String::from("");
    if args % 3 == 0 {
        msg += "Fizz";
    }
    if args % 5 == 0 {
        msg += "Buzz";
    }

    if msg.len() == 0 {
        msg = args.to_string();
    }
    msg
}
