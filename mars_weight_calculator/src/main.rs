/// code along with the Udemy course Learn Rust by Building Real Applications
/// by Lyubomir Gavadinov, https://www.udemy.com/course/rust-fundamentals/
/// Section 3: Building a command line application
/// https://www.udemy.com/course/rust-fundamentals/learn/lecture/20772634#overview
/// 
/// not much coding here but the videos are a welcome refresher 
/// for ownership and borrowing

use std::io;

fn main() {
    let mut input = String::new();

    println!("Please user, please tell us about your weight (in kg): ");
    
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    println!("input we got: {}", weight);
    dbg!(weight);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81 ) * 3.711
}