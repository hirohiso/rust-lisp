
pub struct Enviroments<'a>{
    functions :Vec<(&'a str, fn(&[i32]) -> i32)>,
}

#[test]
fn test(){
    let mut env = Enviroments  {functions:vec![]};
    env.functions.push(("+", |args| args[0] + args[1]));
    env.functions.push(("-", |args| args[0] - args[1]));
    env.functions.push(("*", |args| args[0] * args[1]));
    env.functions.push(("/", |args| args[0] / args[1]));

}