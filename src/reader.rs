use regex::Regex;

fn read_str(src : &str) {
    tokenize(src);

}

fn tokenize(src : &str){
    //[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)
    let re = Regex::new(r##"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"##).unwrap();
    for caps in re.captures_iter(src) {
        println!("token: {:?}", caps.get(1).unwrap().as_str());
    }
}


#[test]
fn comfirm_tokenizer(){
    tokenize("()");
    tokenize("(+ 2 (* 3 4))");
    tokenize("(+ 122 (* 3 4))");
}
