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
        let _ret = read_list(token_list);
    }
    return result;
}

fn read_atom(token_list : Vec<&str>) -> LispCell{
    return LispCell::None;
}

fn read_list(token_list : Vec<&str>) -> LispCell{
    //token_listから一つtokenをとる
    //)以外の場合はrea_formを読み出す
    if token_list[0] == ")"{
        return LispCell::None;
    }else{
        let mut list = LispCell::List{values: Vec::new()};
        let _ret = read_form(token_list);
    }
    return LispCell::None;
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
