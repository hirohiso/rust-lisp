use crate::types::LispCell;

pub fn pr_str(target :LispCell){
    match target {
        LispCell::Number(num) => print!(" {} ",num),
        LispCell::Symbol(sym) => print!(" {} ",sym),
        LispCell::List { values } => {
            print!("(");
            values.into_iter().for_each(|e| pr_str(e));
            print!(")");
        },
        LispCell::None => print!(""),
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
    pr_str(list1);
}