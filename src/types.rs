fn make_number(value : i32) -> Box<LispCell>{
    Box::new(LispCell::Number(value))
}

fn make_symbol(symbol:&str )->Box<LispCell>{
    Box::new(LispCell::Symbol(symbol.to_string()))
}

fn make_list(car:Box<LispCell>,cdr:Box<LispCell>)->Box<LispCell>{
    Box::new(LispCell::List{car,cdr})
}

//Enumで定義したほうが楽そう。。。。
#[derive(Debug,PartialEq)]
enum LispCell{
    Number(i32),
    Symbol(String),
    List{
        car: Box<LispCell>,
        cdr: Box<LispCell>
    },
    None
}

//=====================
//test
//======================
#[test]
fn number_test(){
    let atom = make_number(32);
    if let LispCell::Number(v) = *atom{
        assert_eq!(v ,32);
    }
}
#[test]
fn symbol_test(){
    let atom = make_symbol("test");
    if let LispCell::Symbol(v) = *atom{
        assert_eq!(v ,"test");
    }
}
#[test]
fn list_test(){
    let list = make_list(make_number(10), make_number(13));
    if let LispCell::List{car:v1,cdr:v2} = *list{
        if let LispCell::Number(v) = *v1{
            assert_eq!(v ,10);
        }

        if let LispCell::Number(v) = *v2{
            assert_eq!(v ,13);
        }
        
    }
}
