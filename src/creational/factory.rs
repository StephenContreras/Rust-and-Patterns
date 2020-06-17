pub trait Beverage{
    fn temperature(&self) -> i32;
    fn is_hot(&self) -> bool{
        if self.temperature() > 80 {
            return true;
        }
        else{
            return false;
        }
    }
}

pub struct Tea {
    temperature : i32
}

impl Beverage for Tea{
    fn temperature(&self) -> i32{
        self.temperature
    }
}

pub fn run_factory(){
    println!("FACTORY");
    let tea = Tea {
        temperature : 75 
    };
    println!("Tea is hot?: {}", tea.is_hot());
}


