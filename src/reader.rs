use regex::Regex;
use crate::types::LispCell;

fn read_str(src : &str) {
    tokenize(src);

}

fn tokenize(src : &str) -> Vec<&str>{
    //[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)
    let re = Regex::new(r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##).unwrap();
    re.captures_iter(src).map(|e| e.get(1).unwrap().as_str()).collect()
}

fn read_form(token_list : Vec<&str>) -> Vec<LispCell>{
    let result = Vec::new();
    if token_list[0] == "("{
        let mut list = LispCell::List{values: Vec::new()};
    }
    return result;
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
