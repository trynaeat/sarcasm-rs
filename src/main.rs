use std::io;
use std::error;
use rand::prelude::*;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let lines = io::stdin().lines();
    let mut rng = rand::rng();
    for line in lines {
        let randomized = line?.chars().map(|c| {
            let random_val = rng.random_range(0..100);
            if random_val > 50 {
                return c.to_uppercase().collect::<Vec<_>>()[0];
            } else {
                return c.to_lowercase().collect::<Vec<_>>()[0];
            }
        })
        .collect::<String>();
        println!("{}", randomized);
    }
    return Ok(());
}
