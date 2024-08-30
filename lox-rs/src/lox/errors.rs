#[derive(Debug)]
pub enum LanguageError {
    SyntaxError(String),
    ParserError(String),
    TypeError(String)

}
