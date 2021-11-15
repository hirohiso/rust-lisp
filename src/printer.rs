use crate::types::LispCell;

pub fn pr_str(target :LispCell)->String{
    match target {
        LispCell::Number(num) => return num.to_string(),
        LispCell::Symbol(sym) => return sym,
        LispCell::List { values } => {
            let arr = values.into_iter().map(|e|pr_str(e)).collect::<Vec<_>>().join(" ");
            return format!("({})",arr);
        },
        LispCell::None => return "".to_string(),
    }
}

#[test]
fn pr_str_test(){
    let symbol1 = LispCell::Symbol("*".to_string());
    let symbol2 = LispCell::Number(1);
    let symbol3 = LispCell::Symbol("+".to_string());
    let symbol4 = LispCell::Number(32);
    let symbol5 = LispCell::Number(4);

    let list2 = LispCell::List {
        values: vec![symbol3, symbol4, symbol5],
    };
    let list1 = LispCell::List {
        values: vec![symbol1, symbol2, list2],
    };
    let acual = pr_str(list1);
    assert_eq!(acual,"(* 1 (+ 32 4))")
}