use std::{io, ops::Deref};

use reader::read_str;
use printer::pr_str;
use types::LispCell;

mod reader;
mod printer;
mod types;
mod eval;

fn main() {
    let enviroment = |sym:String,x:i32,y:i32|->i32{
        match sym.as_str() {
            "+" => return x + y,
            "-" => return x - y,
            "*" => return x * y ,
            "/" => return x / y,
            _ => return 0,
        }
    };

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
    let result = pr_str(out);
    println!("{}",result);
}




