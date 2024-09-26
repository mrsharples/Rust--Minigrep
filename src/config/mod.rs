pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub write_to_file: bool,
}

impl Config{
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        
        args.next();

        let ( 
            query,
            file_path,
            mut ignore_case,
            mut write_to_file
        ) = (
            match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string."),
            },
            match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path."),
            },
            false,
            false
        );

        for arg in args {
            if arg == String::from("ignore_case") { ignore_case = true }
            else if arg == String::from("write") {write_to_file = true }
        }
        
        Ok(Config { 
            query, 
            file_path,
            ignore_case, 
            write_to_file,
        })
    }
}
