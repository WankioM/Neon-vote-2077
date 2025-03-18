mod candidates;
mod vote;
mod results;

use std::io;
use colored::*;
use candidates::*;
use vote::*;
use results::*;

fn main() {
    let mut votes: Vec<Vote> = Vec::new();
    let mut voter_count = 1;

    loop {
        println!("{}", "🌆 Welcome to Neon Vote 2077! 🗳️".magenta().bold());
        println!("{}", "Cyberpunk City needs a new mayor. Who will win?".yellow());

        display_candidates();
        println!("{}", "Enter the number of your chosen candidate:".bold());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "❌ Invalid input. Please enter a number.".red().bold());
                continue;
            }
        };

        if choice == 5 {
            break; // Exit and show results
        }

        match get_candidate(choice) {
            Some(candidate) => {
                record_vote(&mut votes, voter_count, candidate.clone());
                confirm_vote(&candidate);
                voter_count += 1;
            }
            None => println!("{}", "❌ Invalid choice. Try again.".red()),
        }
    }

    display_results(&votes);
}
