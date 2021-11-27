

#[derive(Debug,PartialEq,Clone)]
pub enum LispCell{
    Number(i32),
    Symbol(String),
    List{
        values: Vec<LispCell>,
    },
    None
}

impl LispCell {
    pub fn to_int(&self)->Option<i32>{
        if let LispCell::Number(val) = &self{
            return Some(val.clone());
        }
        return None;
    }
}

//=====================
//test
//======================
#[test]
fn number_test(){
    
}

