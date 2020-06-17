mod creational;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // let choice  = &args[1].to_lowercase().as_str();
        match args[1].to_lowercase().as_str() {
            "factory" => creational::factory::run_factory(),
            _ => println!("Unknown input")
        }
        
    }

}
