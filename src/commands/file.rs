use std::{fs, io};

use crate::preprocessor::Preprocessor;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct FileCommand {
    input: String,
    output: String,
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    define: Vec<String>,
}

impl FileCommand {
    pub async fn run(&self) {
        let preprocessor = Preprocessor::new(self.define.clone());

        let path = self.input.clone();
        let output = self.output.replace(&self.input, &self.output);
        let raw = fs::read_to_string(&path).unwrap();
        let res = preprocessor.process_file(&raw);

        if res.is_err() {
            panic!("Error: {}", res.err().unwrap());
        }

        fs::create_dir_all(output.replace(path.as_str(), "")).unwrap();
        let mut file = fs::File::create(output).unwrap();
        io::Write::write_all(&mut file, res.unwrap().as_bytes()).unwrap();
    }
}
