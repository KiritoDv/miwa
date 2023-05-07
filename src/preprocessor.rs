use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub struct Preprocessor {
    context: gpp::Context,
}

impl Preprocessor {
    pub fn new(defines: Vec<String>) -> Self {
        let mut context = gpp::Context::new();
        for define in defines {
            let mut parts = define.split('=');
            let name = parts.next().unwrap();
            if let Some(value) = parts.next() {
                context
                    .macros
                    .insert(name.to_owned(), format!("\"{}\"", value));
            } else {
                context.macros.insert(name.to_owned(), "1".to_owned());
            }
        }
        Preprocessor { context }
    }
    pub fn same_file(file1: &Path, file2: &Path) -> Result<bool, std::io::Error> {
        let f1 = File::open(file1)?;
        let f2 = File::open(file2)?;

        // Check if file sizes are different
        if f1.metadata().unwrap().len() != f2.metadata().unwrap().len() {
            return Ok(false);
        }

        // Use buf readers since they are much faster
        let f1 = BufReader::new(f1);
        let f2 = BufReader::new(f2);

        // Do a byte to byte comparison of the two files
        for (b1, b2) in f1.bytes().zip(f2.bytes()) {
            if b1.unwrap() != b2.unwrap() {
                return Ok(false);
            }
        }

        Ok(true)
    }
    pub fn process_file(mut self, path: &str) -> Result<String, String> {
        let res = gpp::process_file(path, &mut self.context);
        if res.is_err() {
            return Err(format!("Error: {}", res.err().unwrap()));
        }
        Ok(res.unwrap())
    }
}

impl AsRef<Preprocessor> for Preprocessor {
    fn as_ref(&self) -> &Preprocessor {
        self
    }
}
