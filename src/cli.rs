use std::Env;
use std::CLI;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = "cli.rs";
    let mut file = File::open(path).unwrap();
}

fn main() {
    let args = CLI::from_args();
    let path = Path::new(&args.path);
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    let mut line = lines.next().unwrap();
    let mut count = 0;
    while line.len() > 0 {
        count += 1;
        line = lines.next().unwrap();
    }
    println!("{}", count);
}

#[derive(Debug)]
struct CLI {
    path: String,
}
#[derive(Debug)]
struct Args {
    path: String,
}
impl CLI {
    fn from_args() -> Args {
        Args {
            path: Env::args().nth(1).unwrap(),
        }
    }
}

impl Args {
    fn new(args: &[String]) -> Args {
        Args {
            path: args[1].clone(),
        }
    }
}