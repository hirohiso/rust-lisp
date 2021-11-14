use std::str::FromStr;

use crate::types::LispCell;
use regex::{NoExpand, Regex};

fn read_str(src: &str) {
    tokenize(src);
}

fn tokenize(src: &str) -> Vec<&str> {
    //[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)
    let re =
        Regex::new(r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##)
            .unwrap();
    re.captures_iter(src)
        .map(|e| e.get(1).unwrap().as_str())
        .collect()
}

struct readerObj<'a> {
    ptr: usize,
    token_list: Vec<&'a str>,
}

impl<'a> readerObj<'a> {
    fn peek(&self) -> Option<&str> {
        if self.ptr >= self.token_list.len() {
            return None;
        }
        Some(self.token_list[self.ptr])
    }

    fn pop(&mut self) -> Option<&str> {
        if self.ptr >= self.token_list.len() {
            return None;
        }
        let ret = self.token_list[self.ptr];
        self.ptr += 1;
        Some(ret)
    }

    fn read_form(&mut self) -> LispCell {
        if self.peek().unwrap() == "(" {
            let _ = self.pop().unwrap();
            return self.read_list();
        } else {
            return self.read_atom();
        }
    }

    fn read_atom(&mut self) -> LispCell {
        let sym = self.pop().unwrap();
        match sym.parse::<i32>() {
            Ok(num) => return LispCell::Number(num),
            Err(_) =>return LispCell::Symbol(String::from_str(sym).unwrap()),
        }
    }

    fn read_list(&mut self) -> LispCell {
        //token_listから一つtokenをとる
        //)以外の場合はrea_formを読み出す
        let mut content = Vec::new();
        while self.peek().is_some() {
            if self.peek().unwrap() == ")" {
                let _ = self.pop().unwrap();
                return LispCell::List { values: content };
            } else {
                let ret = self.read_form();
                content.push(ret);
            }
        }
        //エラーの方がいい
        return LispCell::None;
    }
}

#[test]
fn comfirm_tokenizer() {
    let actual = tokenize("()");
    assert_eq!(vec!["(", ")"], actual);
    let actual = tokenize("(+ 2 (* 3 4))");
    assert_eq!(vec!["(", "+", "2", "(", "*", "3", "4", ")", ")"], actual);
    let actual = tokenize("(+ 122 (3))");
    assert_eq!(vec!["(", "+", "122", "(", "3", ")", ")"], actual);
}

#[test]
fn read_object_test() {
    let mut sut = readerObj {
        ptr: 0,
        token_list: vec!["(", "1", "22", ")"],
    };

    assert_eq!(sut.peek().unwrap(), "(");
    assert_eq!(sut.pop().unwrap(), "(");
    assert_eq!(sut.pop().unwrap(), "1");
    assert_eq!(sut.pop().unwrap(), "22");
    assert_eq!(sut.pop().unwrap(), ")");
    assert_eq!(sut.pop(), None);
}

#[test]
fn read_object_atom_test() {
    let mut sut = readerObj {
        ptr: 0,
        token_list: vec!["1"],
    };

    let ret = sut.read_atom();
    assert_eq!(ret, LispCell::Number(1));
}

#[test]
fn read_object_list_test() {
    let mut sut = readerObj {
        ptr: 0,
        token_list: vec!["1", "3", ")"],
    };

    let ret = sut.read_list();
    let symbol1 = LispCell::Number(1);
    let symbol2 = LispCell::Number(3);
    assert_eq!(
        ret,
        LispCell::List {
            values: vec![symbol1, symbol2]
        }
    );
}

#[test]
fn read_object_form() {
    let mut sut = readerObj {
        ptr: 0,
        token_list: vec!["(", "1", "3", ")"],
    };

    let ret = sut.read_form();
    let symbol1 = LispCell::Number(1);
    let symbol2 = LispCell::Number(3);
    assert_eq!(
        ret,
        LispCell::List {
            values: vec![symbol1, symbol2]
        }
    );

    let mut sut = readerObj {
        ptr: 0,
        token_list: vec!["(", "*", "1", "(", "+", "3", "4", ")", ")"],
    };
    let symbol1 = LispCell::Symbol("*".to_string());
    let symbol2 = LispCell::Number(1);
    let symbol3 = LispCell::Symbol("+".to_string());
    let symbol4 = LispCell::Number(3);
    let symbol5 = LispCell::Number(4);

    let list2 = LispCell::List {
        values: vec![symbol3, symbol4, symbol5],
    };
    let list1 = LispCell::List {
        values: vec![symbol1, symbol2, list2],
    };
    let ret = sut.read_form();
    assert_eq!(ret, list1);
}
