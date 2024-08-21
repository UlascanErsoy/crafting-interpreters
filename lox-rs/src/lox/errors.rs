#[derive(Debug)]
pub enum LanguageError {
    SyntaxError(String),
    ParserError(String)
}
