use std::io::{self, Write};

fn main() {

    let mut weight = String::new();
    let mut height = String::new();
    let mut age = String::new();

    print!("Please enter your age: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age: u32 = age.trim().parse().expect("Please input a number!");

    print!("Please enter your weight in lbs: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let weight: f32 = weight.trim().parse().expect("Please input a number!");

    print!("Please enter your height in inches: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: f32 = height.trim().parse().expect("Please input a number!");

    calc_bmr(age, convert_height(height), convert_weight(weight));
}

fn convert_weight(lbs: f32) -> f32 {
    const KG_PER_LB: f32 = 0.45359;
    return lbs * KG_PER_LB;
}

fn convert_height(inches: f32) -> f32 {
    const CM_PER_IN: f32 = 2.54;
    return inches * CM_PER_IN;
}

fn calc_bmr(age: u32, height: f32, weight: f32) {
    println!("You are {} years old, {} cm tall, and weigh {} kg", age, height, weight);
    let age: f32 = age as f32;
    let bmr = (10.0 * weight) + (6.25 * height) - (5.0 * age) + 5.0;
    println!("Your Basal Metabolic Rate (BMR) is {} kcal/day", bmr);
}
