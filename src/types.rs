

#[derive(Debug,PartialEq,Clone)]
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

