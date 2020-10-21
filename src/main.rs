use std::io;

fn main() {

    const KG_PER_LB: f32 = 0.45359;

    let mut weight = String::new();

    println!("Please enter your weight in lbs: ");
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let weight: f32 = weight.trim().parse().expect("Please input a number!");

    println!("\nWeight is {} lbs.", weight);
    println!("\nWeight is {} kgs.", weight * KG_PER_LB);
}
