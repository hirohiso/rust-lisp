use std::str::FromStr;

use regex::{NoExpand, Regex};
use crate::types::LispCell;

fn read_str(src : &str) {
    tokenize(src);

}

fn tokenize(src : &str) -> Vec<&str>{
    //[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)
    let re = Regex::new(r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##).unwrap();
    re.captures_iter(src).map(|e| e.get(1).unwrap().as_str()).collect()
}



struct readerObj<'a>{
    ptr : usize,
    token_list :Vec<&'a str> ,
}

impl <'a>readerObj<'a> {
    fn peek(&self)->Option<&str>{
        if self.ptr >= self.token_list.len(){
            return None;
        }
        Some(self.token_list[self.ptr])
    }

    fn pop(&mut self)->Option<&str>{
        if self.ptr >= self.token_list.len(){
            return None;
        }
        let ret = self.token_list[self.ptr];
        self.ptr += 1;
        Some(ret)
    }

    fn read_form(&mut self) -> Vec<LispCell>{
        let mut result = Vec::new();
        if self.peek().unwrap()  == "("{
            let _ = self.pop().unwrap() ;
            let ret = self.read_list();
            let mut list = LispCell::List{values: vec![ret]};
            result.push(list);
            return result;
        }
        return result;
    }

    fn read_atom(&mut self) -> LispCell{
        let sym = self.pop().unwrap();
        return LispCell::Symbol(String::from_str(sym).unwrap());
    }


    fn read_list(&mut self) -> LispCell{
        //token_listから一つtokenをとる
        //)以外の場合はrea_formを読み出す
        if self.peek().unwrap() == ")"{
            let _ = self.pop().unwrap() ;
            return LispCell::None;
        }else{
            let mut list = LispCell::List{values: Vec::new()};
            let _ret = self.read_form();
        }
        return LispCell::None;
    }
}


#[test]
fn comfirm_tokenizer(){
    let actual = tokenize("()");
    assert_eq!(vec!["(",")"],actual);
    let actual = tokenize("(+ 2 (* 3 4))");
    assert_eq!(vec!["(","+","2","(","*","3","4",")",")"],actual);
    let actual = tokenize("(+ 122 (3))");
    assert_eq!(vec!["(","+","122","(","3",")",")"],actual);
}

#[test]
fn read_object_test(){
    let mut sut = readerObj{
        ptr : 0,
        token_list : vec!["(","1","22",")"],
    };

    assert_eq!(sut.peek().unwrap(),"(");
    assert_eq!(sut.pop().unwrap(),"(");
    assert_eq!(sut.pop().unwrap(),"1");
    assert_eq!(sut.pop().unwrap(),"22");
    assert_eq!(sut.pop().unwrap(),")");
    assert_eq!(sut.pop(),None);

}
