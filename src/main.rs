fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let choice :&str = &args[1];
    println!("{:?}", choice);

}
