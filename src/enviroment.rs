
pub struct Enviroments<'a>{
    functions :Vec<(&'a str, fn(&[i32]) -> i32)>,
}

impl<'a> Enviroments<'a>{
    pub fn find_func(&self, symbol : &str)-> Option<fn(&[i32]) -> i32>{
        let tapl = self.functions.iter()
        .find(|&&e|e.0 == symbol)
        .map(|&e|e.1);
        return tapl;
    }

    pub fn push(&mut self , tapl :(&'a str, fn(&[i32]) -> i32)){
        self.functions.push(tapl);
    }

    pub fn new()-> Enviroments<'a>{
        Enviroments{functions: Vec::new()}
    }
}

#[test]
fn test(){
    let mut env = Enviroments  {functions:vec![]};
    env.push(("+", |args| args[0] + args[1]));
    env.push(("-", |args| args[0] - args[1]));
    env.push(("*", |args| args[0] * args[1]));
    env.push(("/", |args| args[0] / args[1]));

    let func = env.find_func("*");
    assert!(func.is_some());
    let func = env.find_func("%");
    assert!(func.is_none());

}