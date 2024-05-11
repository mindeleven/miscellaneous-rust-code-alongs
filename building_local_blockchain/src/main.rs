use serde_derive;
use std::{
    io::{
        self, Chain, Write
    }, 
    process,
};

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    // we're letting the user choose what he wants to do
    let mut choice = String::new();

    println!("Let's create a blockchain!");

    println!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    println!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");

    println!("Generating Genesis block!");
    let mut chain = blockchain::Chain::new(
        miner_addr.trim().to_string(), 
        diff
    );

    // building a menu inside a loop
    loop {
        println!("Menu");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty");
        println!("4. Change reward");
        println!("0. Exit");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting");
                process::exit(0);
            },
            1 => {
                // adding new transaction to chain
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                println!("Enter receiver address:");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                println!("Enter amount:");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(), 
                    receiver.trim().to_string(), 
                    amount.trim().parse().unwrap()
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            },
            2 => {
                // mining a block
                println!("Generating block!");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }


            },
            3 => {
                let mut new_diff = String::new();
                println!("Please enter new difficulty:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Update failed"),
                }
            },
            4 => {
                let mut new_reward = String::new();
                println!("Please enter new reward:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Update failed"),
                }

            },
            _ => {
                println!("Invalid choice, please try again")
            }
        }
    }
    

}
