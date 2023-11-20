use clap::{Arg, Command};
use constituency_tree_parser::ConstituencyTree;

fn main() {
    //let tree = ConstituencyTree::parse(input);
    //println!("{:#?}", tree);
    let current_version = env!("CARGO_PKG_VERSION");

    let matches = Command::new("Constituency Tree Parser")
        .version(current_version)
        .author("Bohdan Tsehelnyk")
        .about("Parse text constituency tree into data structure")
        .arg(Arg::new("parse").short('p').help("Parse tree"))
        .get_matches();

    let prase_str = matches.get_one::<String>("parse");
    match prase_str {
        None => println!("No subcommand provided. Use -h for help."),
        Some(input) => {
            match ConstituencyTree::parse(String::from(input)) {
                Ok(tree) => {
                    println!("Parsed Constituency Tree:\n{:#?}", tree);
                    // You can now work with the parsed tree as needed
                }
                Err(err) => {
                    eprintln!("Error parsing constituency tree: {}", err);
                }
            }
        }
    }
}
