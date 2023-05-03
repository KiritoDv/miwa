use std::{fs, io, path::Path};

use crate::preprocessor::Preprocessor;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
pub struct DirectoryCommand {
    input: String,
    output: String,
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    filter: Option<Vec<String>>,
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    define: Vec<String>,
}

fn compare_diff(text: &str, diff: &str) -> bool {
    let text = text.split('\n');
    let mut diff = diff.split('\n');

    for t in text {
        let d = diff.next();
        if d.is_none() || t != d.unwrap() {
            return false;
        }
    }

    true
}

impl DirectoryCommand {
    pub async fn run(&self) {
        let extensions = self.filter.clone();

        for entry in WalkDir::new(self.input.clone())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }

            let ext = path.extension();
            if ext.is_none() {
                continue;
            }

            if extensions.is_some()
                && !extensions
                    .as_ref()
                    .unwrap()
                    .contains(&ext.unwrap().to_str().unwrap().to_owned())
            {
                continue;
            }

            let preprocessor = Preprocessor::new(self.define.clone());
            let output = path.to_str().unwrap().replace(&self.input, &self.output);
            let raw = fs::read_to_string(path).unwrap();
            let res = preprocessor.process_file(&raw);

            if res.is_err() {
                println!("Error: {}", res.err().unwrap());
                continue;
            }

            let processed = res.unwrap();

            if compare_diff(&raw, &processed) {
                println!("Skipping {}", path.to_str().unwrap());
                continue;
            }

            println!("Processing {}", path.to_str().unwrap());

            fs::create_dir_all(
                output
                    .clone()
                    .replace(path.file_name().unwrap().to_str().unwrap(), ""),
            )
            .unwrap();
            let mut file = fs::File::create(&output).unwrap();
            io::Write::write_all(&mut file, processed.as_bytes()).unwrap();

            if Preprocessor::same_file(path, Path::new(&output)).unwrap() {
                fs::remove_file(output).unwrap();
            }
        }
    }
}
