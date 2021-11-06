use std::io;


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
    "test"
}

fn write(out:&str){
    println!("{}",out)
}