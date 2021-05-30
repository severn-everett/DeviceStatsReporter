use crate::lib::runner;

mod lib;

fn main() {
    println!("Running Device Stats Reporter");
    match runner::run() {
        Ok(_) => { println!("Run complete") }
        Err(e) => {
            eprintln!("Error running Device Stats Reporter: {:?}", e);
        }
    };
}