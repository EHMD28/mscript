use mscript::mbuild::lexer::Lexer;

fn main() {
    let line = "set x to 5;";
    let lexer = Lexer::new(line);
    while let Some(token) = lexer.next_token() {
        println!("{token}")
    }
}
