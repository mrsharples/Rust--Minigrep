pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
    pub write_to_file: bool,
}

impl Config<'_>{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = &args[1];
        let file_path = &args[2];

        let (
            ignore_case, 
            write_to_file
        ) = (
            args.contains(&String::from("ignore_case")), 
            args.contains(&String::from("write"))
        );
            

        // let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { 
            query, 
            file_path,
            ignore_case, 
            write_to_file,
        })
    }
}
