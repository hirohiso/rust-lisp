use std::collections::btree_map::Values;



//Enumで定義したほうが楽そう。。。。
#[derive(Debug,PartialEq)]
pub enum LispCell{
    Number(i32),
    Symbol(String),
    List{
        values: Vec<LispCell>,
    },
    None
}

//=====================
//test
//======================
#[test]
fn number_test(){
    
}

