use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "miwa", version)]
struct Args {
    path: String,
    output: Option<String>,
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    define: Vec<String>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut context = gpp::Context::new();
    for define in args.define {
        let mut parts = define.split('=');
        let name = parts.next().unwrap();
        if let Some(value) = parts.next() {
            context.macros.insert(name.to_owned(), format!("\"{}\"", value));
        } else {
            context.macros.insert(name.to_owned(), "1".to_owned());
        }
    }
    let res = gpp::process_file(&args.path, &mut context);
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
        return;
    }
    if let Some(output) = args.output {
        std::fs::create_dir_all(std::path::Path::new(&output).parent().unwrap()).unwrap();
        std::fs::write(output, res.unwrap()).unwrap();
    } else {
        println!("{}", res.unwrap());
    }
}
