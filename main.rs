use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use regex::Regex;
use num_format::{english_number_system, Locale};


fn main() -> Result<(), Box<dyn Error>> {
    let dork_types = read_dork_types("DorkTypes.txt")?;
    let placeholders = find_placeholders(&dork_types);
    let placeholder_values = read_placeholder_values(&placeholders)?;
    let num_combinations = calculate_num_combinations(&placeholder_values);
    let num_modified_dorks = num_combinations * dork_types.len();
    let formatted_num = english_number_system(num_modified_dorks, Locale::en).unwrap();
    println!("Generating {} modified dorks...", formatted_num);
    let mut output_file = File::create("combinations.txt")?;
    let mut modified_dorks = HashSet::new();

    for i in 0..num_combinations {
        let mut combination = Vec::new();
        let mut j = i;
        for (placeholder, values) in &placeholder_values {
            let value = &values[j % values.len()];
            combination.push((placeholder.to_owned(), value.to_owned()));
            j /= values.len();
        }

        for dork_type in &dork_types {
            let mut modified_dork = dork_type.to_owned();
            for (placeholder, value) in &combination {
                modified_dork = modified_dork.replace(&format!("({})", placeholder), value);
            }
            if !modified_dorks.contains(&modified_dork) {
                modified_dorks.insert(modified_dork.clone());
                writeln!(output_file, "{}", modified_dork)?;
            }
        }
    }

    Ok(())
}

fn read_dork_types(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut dork_types = Vec::new();
    for line in reader.lines() {
        dork_types.push(line?);
    }

    Ok(dork_types)
}

fn find_placeholders(dork_types: &[String]) -> HashSet<String> {
    let placeholder_regex = Regex::new(r"\((.*?)\)").unwrap();
    let mut placeholders = HashSet::new();
    for dork_type in dork_types {
        for capture in placeholder_regex.captures_iter(dork_type) {
            placeholders.insert(capture[1].to_owned());
        }
    }
    placeholders
}


fn read_placeholder_values(placeholders: &HashSet<String>) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    let mut placeholder_values = Vec::new();
    for placeholder in placeholders {
        let values = read_dork_types(&format!("{}.txt", placeholder))?;
        placeholder_values.push((placeholder.to_owned(), values));
    }
    Ok(placeholder_values)
}

fn calculate_num_combinations(placeholder_values: &[(String, Vec<String>)]) -> usize {
    placeholder_values
        .iter()
        .map(|(_, values)| values.len())
        .product()
}
