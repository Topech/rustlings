// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.


fn main() {
    println!("{}", call_me());
}


fn call_me() -> &'static str {
    "Called me!"
}
