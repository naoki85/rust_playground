use mysql_query_parser::lexer::tokenizer::Tokenizer;

fn main() {
    let res = Tokenizer::tokenize("select * from users where id = 3;");
    println!("{:?}", res);
}