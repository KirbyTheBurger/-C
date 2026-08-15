use neg_c::lexer::tokenize;

fn main() {
    let input = std::fs::read_to_string("test.nc").unwrap();
    match tokenize(&input) {
        Ok(t) => t.iter().for_each(|t| print!("{:?}, ", t.element)),
        Err(e) => e.iter().for_each(|e| e.report("test.nc")),
    }
}
