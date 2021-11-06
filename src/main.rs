use std::io;
use rand::Rng;

fn get_num() -> u32 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let num: u32 = line.trim()
        .parse()
        .expect("Not a number!");
    return num;
}

fn get_positive_num() -> u32 {
    let num = get_num();
    if num <= 0 {
        panic!("Not a positive number!");
    }
    return num;
}

fn main() {

    println!("Please enter the amount of dice you're rolling:");
    let num_dice = get_positive_num();

    println!("Please enter the number of sides on the dice you are rolling.");
    let sides = get_positive_num();

    println!("Performing a {} d{} roll...", num_dice, sides);
    println!("Results:\n========");

    for _n in 0..num_dice {
        let roll = rand::thread_rng().gen_range(1..(sides+1));
        println!("{}", roll);
    }

    println!("========");

}
