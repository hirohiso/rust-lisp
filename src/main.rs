use std::{io, ops::Deref};

use reader::read_str;
use printer::pr_str;
use types::LispCell;

mod reader;
mod printer;
mod types;

fn main() {
    loop{
        println!("input eval");
        write(eval(read()));
        println!("");
    }
}

fn read() -> LispCell {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    read_str(input.as_str())
}

fn eval(exp : LispCell) -> LispCell{
    exp
}

fn write(out:LispCell){
    pr_str(out);
}




