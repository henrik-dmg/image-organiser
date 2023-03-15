use clap::Parser;

#[derive(Parser)]
struct CIArguments {
    configuration_path: std::path::PathBuf,
}

fn main() {
    let args = CIArguments::parse();

    println!("Configuration path: {}", args.configuration_path.display());

    let content = std::fs::read_to_string(&args.configuration_path)
        .expect("Could not read configuration file.");

    for line in content.lines() {
        println!("{}", line);
    }
}
