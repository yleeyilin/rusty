use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    /*
    If the last line of a block doesn’t end with a semicolon (;), 
    the value of that expression becomes the return value of the block.
    */
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    // mini grep func
    let content = std::fs::read_to_string(&args.path).expect("could not read file"); // reads whole file into mem
    for ln in content.lines() {
        if ln.contains(&args.pattern) {
            println!("{}", ln)
        }
    }
}
