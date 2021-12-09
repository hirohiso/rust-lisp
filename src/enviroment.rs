use crate::types::LispCell;

pub struct Enviroments<'a>{
    functions :Vec<(&'a str, fn(&[i32]) -> i32)>,
    outer : Option<&'a Enviroments<'a>>,
    data : Vec<(String, LispCell)>,
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

    pub fn set(&mut self,symbol : String,value :LispCell){
        self.data.push((symbol,value));
    }

    pub fn find(& self,symbol : &str)->&Enviroments{
        //dataにsymbolがあるか確認する
        let value = self.data.iter().find(|e|e.0.as_str() == symbol).map(|e|e.1.clone());
        //ないならouterのfindを呼ぶ
        if value.is_none() {
            let outer = self.outer;
            if let Some(env) = outer {
                return env.find(symbol);
            }
            panic!("Not Found Enviroment")
        }
        return self;
    }
    pub fn get(& self,symbol:&str)->LispCell{
        //findを呼んでEnviromentを得る
        //enviromentのdataからvalueを返却する
        //存在しない場合は例外を投げる
        return LispCell::None;
    }

    pub fn new()-> Enviroments<'a>{
        Enviroments{functions: Vec::new(),outer:None,data:Vec::new()}
    }
}

#[test]
fn test(){
    let mut env = Enviroments::new();
    env.push(("+", |args| args[0] + args[1]));
    env.push(("-", |args| args[0] - args[1]));
    env.push(("*", |args| args[0] * args[1]));
    env.push(("/", |args| args[0] / args[1]));

    let func = env.find_func("*");
    assert!(func.is_some());
    let func = env.find_func("%");
    assert!(func.is_none());

}