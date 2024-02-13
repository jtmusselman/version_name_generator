// James Musselman
// Released under AGPL-v3.0-or-later
// This program generates a version name using a random alliterative animal and adjective from their respective files.

use std::fs::File;
use std::io::{self, BufRead};
use inflector::Inflector;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    // Open the files
    let animal_file = File::open("animals.txt")?;
    let adj_file = File::open("adj.txt")?;
   
    let animal_reader = io::BufReader::new(animal_file);
    let adj_reader = io::BufReader::new(adj_file);

    // Read each line into a vector
    let animal_lines: Vec<String> = animal_reader.lines().filter_map(|line| line.ok()).collect();
    let adj_lines: Vec<String> = adj_reader.lines().filter_map(|line| line.ok()).collect();

    // Check if the animal file is empty
    if animal_lines.is_empty() {
        println!("Animal file is empty");
        return Ok(());
    }

    // Check if the adjective file is empty
    if adj_lines.is_empty() {
        println!("Adjective file is empty");
        return Ok(());
    }

    let mut rng = thread_rng();


    loop {
        // Generate random lines
        let animal = animal_lines.choose(&mut rng).unwrap();
        let adjective = adj_lines.choose(&mut rng).unwrap().to_title_case();

        // Check if the first character of adjective and animal match
        if animal.chars().next() == adjective.chars().next() {
            // Print the matched version name
            println!("\n{} {}\n", adjective, animal);
            
            println!("Are you satisfied? (y/n)");
    
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
    
            // Trim leading/trailing whitespaces
            let input = input.trim();
    
            if input.eq_ignore_ascii_case("y") {
                break;
            } else if input.eq_ignore_ascii_case("n") {
                println!("Generating new output...")
            
            } else {
                println!("Invalid input. Please enter 'y' or 'n'.");
                continue;
            }
        }
    }

    Ok(())
}
