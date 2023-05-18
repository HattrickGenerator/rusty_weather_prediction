use crate::forecast::forecast::prediction::Potatoe;

pub mod forecast;

fn main() {
    let plant = Potatoe {};    
    println!("I'm growing {:?}!", plant);
}