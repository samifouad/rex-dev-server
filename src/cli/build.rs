use crate::cli::Args;

pub fn cmd (context: &Args) {
    for (key, value) in context.params.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("optimize project for production");
}