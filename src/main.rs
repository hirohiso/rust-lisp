use std::{io, ops::Deref};
mod reader;
mod types;

fn main() {
    loop{
        write(eval(read().as_str()));
    }
}

fn read<'a>() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input
}

fn eval(exp : &str) -> &str{
    exp
}

fn write(out:&str){
    println!("{}",out)
}




