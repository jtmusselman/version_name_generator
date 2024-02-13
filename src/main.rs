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
            println!("Your new Version Name:\n{} {}", adjective, animal);
            break;
        }
    }

    Ok(())
}
