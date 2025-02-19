use regex::Regex;
use std::env;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn intro() {
    println!("Welcome to Doc bot~ It will attempt to parse a file and generate a rough review documents based on it.");
    println!("Usage follows: doc <file> <output filename>");
}
static INTRO: &str = "# Title here \n
Built by Docbot, Reviewed by XX\n\n
## TL;DR\n\n
## Static Code Observation\n
### Headers\n
";
static FUNCTIONS: &str = "### Functions\n\n";

static WINDOWSFUNC: &str = "### Windows API calls\n\n";

fn main() {
    intro();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect use docbot <file>")
    }
    let file_path = &args[1];
    let stringname = &args[2];
    let path = Path::new(stringname);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create to {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(INTRO.as_bytes()) {
        Err(why) => panic!("Cannot write to {}: {}", display, why),
        Ok(file) => println!("Successfully wrote Intro to {} ", display),
    }

    println!("Using {} as requested.", file_path);
    let reviewcode: String = fs::read_to_string(file_path).expect("Could not read String.");
    //println!("{}", reviewcode);
    let output = String::new();
    let headers = Regex::new(r"(#include )(\S+)").expect("Could not make regex");
    //let head: Vec<_> = headers.find_iter(&reviewcode).map(|c| c.as_str()).collect();
    let mut head2: Vec<(&str, &str)> = vec![];

    for (_, [f1, f2]) in headers
        .captures_iter(reviewcode.as_str())
        .map(|caps| caps.extract())
    {
        head2.push((f1, f2))
    }
    for a in &head2 {
        write!(file, "#### {} \n```{}{}```\n\n", a.1, a.0, a.1).expect("Failed to write headers");
    }

    let functions = Regex::new(r"(char\*|void|int|void\*|VOID\*|VOID)\s(\w+)\(.*\)")
        .expect("Failed to regex functions");

    match file.write_all(FUNCTIONS.as_bytes()) {
        Err(why) => panic!("Cannot write to {}: {}", display, why),
        Ok(file) => println!("Successfully wrote Function Section to {} ", display),
    }

    let mut func: Vec<(&str, &str)> = vec![];
    for (_, [r1, r2]) in functions
        .captures_iter(reviewcode.as_str())
        .map(|carp| carp.extract())
    {
        func.push((r1, r2));
    }
    for b in &func {
        write!(file, "#### {} \n```{}{}```\n\n", b.1, b.0, b.1)
            .expect("Failed to write functions to file.");
    }

    match file.write_all(WINDOWSFUNC.as_bytes()) {
        Err(why) => panic!("Cannot write to {}: {}", display, why),
        Ok(file) => println!("Successfully wrote Function Section to {} ", display),
    }
    let winapicall = Regex::new(r"(\w+)\$(\w+)").expect("Failed to regex windows calls");

    let mut winfunc: Vec<(&str, &str)> = vec![];

    for (_, [c1, c2]) in winapicall
        .captures_iter(reviewcode.as_str())
        .map(|ca| ca.extract())
    {
        winfunc.push((c1, c2));
    }
    for c in &winfunc {
        write!(file, "#### {} \n```{}{}```\n\n", c.1, c.0, c.1)
            .expect("Failed to write functions to file.");
    }
}
