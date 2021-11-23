use std::{env, io, ops::Deref};

use reader::read_str;
use printer::pr_str;
use types::LispCell;

mod reader;
mod printer;
mod types;
mod eval;

fn main() {
    let mut enviroment: Vec<(&str,fn(Vec<i32>)->i32)> = vec![
        ("+",|args|{args[0] + args[1]} ),
        ("-",|args|{args[0] - args[1]} ),
        ("*",|args|{args[0] * args[1]} ),
        ("/",|args|{args[0] / args[1]} ),
        ];

    loop{
        println!("input eval");
        write(eval(read(),&mut enviroment));
        println!("");
    }
}

fn read() -> LispCell {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    read_str(input.as_str())
}

fn eval(exp : LispCell,enviroment:&mut Vec<(&str,fn(Vec<i32>)->i32)>) -> LispCell{
    if let LispCell::List{values} = exp {
        if values.len() == 0 {
            return LispCell::List{values:values};
        }else{
            //SYMBOLに合わせて関数を取得する
            let tapl = (*enviroment).iter().find(|e|(**e).0 == "+");
            if let Some(val) = tapl{
                let func = val.1;               
                //引数をint型で取得する
                let args = vec![1,2];
                let ret = func(args);
                return LispCell::Number(ret);
            }

        }
        return LispCell::None;
    }
    return eval_ast(exp, enviroment)
}

fn eval_ast(exp : LispCell,enviroment:&mut Vec<(&str,fn(Vec<i32>)->i32)>) -> LispCell{
    match exp {
        LispCell::Symbol(_) => {
            return exp;
        },
        LispCell::List {..} => {
            return exp;
        },
        _ => (),
    }
    return exp;
}

fn write(out:LispCell){
    let result = pr_str(out);
    println!("{}",result);
}




