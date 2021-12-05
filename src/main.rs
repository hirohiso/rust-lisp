use std::{env, io, ops::Deref};

use printer::pr_str;
use reader::read_str;
use types::LispCell;
use enviroment::Enviroments;

mod eval;
mod printer;
mod reader;
mod types;
mod enviroment;

fn main() {
    let mut env = Enviroments::new();
    env.push(("+", |args| args[0] + args[1]));
    env.push(("-", |args| args[0] - args[1]));
    env.push(("*", |args| args[0] * args[1]));
    env.push(("/", |args| args[0] / args[1]));

    loop {
        println!("input eval");
        write(eval(read(),&mut env));
        println!("");
    }
}

fn read() -> LispCell {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    read_str(input.as_str())
}

fn eval(exp: LispCell, env :&mut Enviroments) -> LispCell {
    if let LispCell::List { values } = exp {
        if values.len() == 0 {
            return LispCell::List { values: values };
        } else {
            let exp = eval_ast(LispCell::List { values: values },env);
            if let LispCell::List { values } = exp {
                //SYMBOLに合わせて関数を取得する
                let sym = &values[0];
                if let LispCell::Symbol(sym) = sym {
                    let func = (*env).find_func(sym.as_str());
                    if let Some(func) = func {
                        //引数をint型で取得する
                        let iter: Vec<i32> = values
                            .iter()
                            .filter_map(|cell| cell.to_int())
                            .collect();
                        let ret = func(&iter);
                        return LispCell::Number(ret);
                    }
                }
            }
        }
        return LispCell::None;
    }
    return eval_ast(exp,env);
}

fn eval_ast(exp: LispCell, env :&mut Enviroments) -> LispCell {
    match exp {
        LispCell::Symbol(_) => {
            return exp;
        }
        LispCell::List {  values } => {
            let new_values = values.iter().map(|e|eval(e.clone(),env)).collect();
            return LispCell::List{values: new_values}; 
            //return LispCell::None;
        }
        _ => (),
    }
    return exp;
}

fn write(out: LispCell) {
    let result = pr_str(out);
    println!("{}", result);
}


#[test]
fn eval_test(){
    let mut env = Enviroments::new();
    env.push(("+", |args| args[0] + args[1]));
    env.push(("-", |args| args[0] - args[1]));
    env.push(("*", |args| args[0] * args[1]));
    env.push(("/", |args| args[0] / args[1]));

    let cell = read_str("(+ 1 3)");
    let act = eval(cell, &mut env);
    let exp = LispCell::Number(4);
    assert_eq!(exp,act);

    let cell = read_str("(* 2 3)");
    let act = eval(cell,&mut env);
    let exp = LispCell::Number(6);
    assert_eq!(exp,act);

    let cell = read_str("(* 2 (+ 2 4))");
    let act = eval(cell,&mut env);
    let exp = LispCell::Number(12);
    assert_eq!(exp,act);

    let cell = read_str("(* (+ 1 2) (+ 2 (- 7 (/ 10 2))))");
    let act = eval(cell,&mut env);
    let exp = LispCell::Number(12);
    assert_eq!(exp,act);
}