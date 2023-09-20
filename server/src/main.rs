fn main() {
    println!("Hello, world! from server");
    let serve = env!("CLIENT_DIST");
    println!("going to serve content from {serve}");
}
