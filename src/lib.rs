use std::{ 
    error::Error, 
    fs::{self, File}, 
    io::Write, 
    path::PathBuf 
};

pub mod config;

fn search<'a>(
    query: &str, 
    contents: &'a str,
    ignore_case: bool,
) -> Vec<(usize, String)> {
    let query = if ignore_case { &query.to_lowercase() } else { query };
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if ignore_case && line.to_lowercase().contains(&query) {
            results.push((i + 1, String::from(line)));
        } else if line.contains(query) {
            results.push((i + 1, String::from(line)));
        }
    }

    results
}

fn write_to_file(file_name: &str, contents: &Vec<(usize, String)>) -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from("./results/").join(file_name);

    let mut file = File::create(&file_path).unwrap_or_else(|err| {
            panic!("Problem creating file: {err}");
    });

    for (i, line) in contents {
        write!(file, "{i}: {line}\n")?;
    }

    Ok(())
}


pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(config.query, &contents, config.ignore_case);

    if config.write_to_file {

        let file_name = &format!("{}-{}", config.query, config.file_path);
        write_to_file(file_name, &results)?;

    } else {
        println!("Search term: {}", config.query);
        println!("In file: {}", config.file_path);
        
        for (index, line) in &results {
            println!("{index}: {line}")
        }
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
            vec![(2, String::from("safe, fast, productive."))], 
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
            vec![(1, String::from("Rust:")), (4, String::from("Trust me."))],
            search(query, contents, true)
        );
    }

}