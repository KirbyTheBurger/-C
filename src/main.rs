use neg_c::lexer::tokenize;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{:?}", tokenize(&input).unwrap());
}
