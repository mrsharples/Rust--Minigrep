use std::{ 
    error::Error, 
    fs::{self, File}, 
    io::Write, 
    path::PathBuf 
};

use config::Config;

pub mod config;

fn search<'a>(
    query: &str, 
    contents: &'a str,
    ignore_case: bool,
) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| ignore_case && line.to_lowercase().contains(&query.to_lowercase()) || line.contains(query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let file_path = PathBuf::from("./files/").join(file_name);

    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}

fn write_to_file(config: &Config, contents: &Vec<(usize, &str)>) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}-{}", config.query, config.file_path);
    let file_path = PathBuf::from("./results/").join(file_name);

    let mut file = File::create(&file_path)?;

    for (i, line) in contents {
        write!(file, "{i}: {line}\n",)?;
    }

    println!("Saved to file in results folder.");

    Ok(())
}

fn print_results(config: &Config, results: &Vec<(usize, &str)>) {
    println!("Search term: {}", config.query);
    println!("In file: {}", config.file_path);
    
    for (index, line) in results {
        println!("{index}: {line}")
    }
}


pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_contents = read_file(&config.file_path)?;
    let results = search(&config.query, &file_contents, config.ignore_case);
    
    print_results(&config, &results);

    if config.write_to_file {
        write_to_file(&config, &results)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![(2, "safe, fast, productive.")], 
            search(query, contents, false));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search(query, contents, true)
        );
    }

}