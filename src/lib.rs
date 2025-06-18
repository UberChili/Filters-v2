use std::error::Error;

pub struct Config {
    file_path: String,
    out_file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path".into()),
        };

        let out_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an ouput file path".into()),
        };

        let config = Config {
            file_path,
            out_file_path,
        };

        Ok(config)
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn out_file_path(&self) -> &String {
        &self.out_file_path
    }
}

// This is where we should do most of the work
pub fn run() {
    todo!()
}
