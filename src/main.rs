use crate::forecast::forecast::Asparagus;

pub mod forecast;

fn main() {
    let plant = Asparagus {};    
    println!("I'm growing {:?}!", plant);
}