use std::{env, fs, io};
use std::io::Write;

#[derive(Debug)]
enum TokenType {
    // Single-char tokens
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two char tokens
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    //Literals
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),
    //Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32
}

#[derive(Debug)]
enum LanguageError {
    SyntaxError(String)
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
    errors: Vec<LanguageError>
}

impl Scanner {
    fn new(source: String)  -> Scanner {
       Scanner {
           source,
           tokens: Vec::new(),
           start: 0,
           current: 0,
           line:1,
           errors: Vec::new()
       }
    }
    fn add_token(&mut self, token: TokenType) {
       self.tokens.push(
           Token {
               token_type: token,
               lexeme: String::from(&self.source[self.start..self.current]),
               line: self.line
           });

       self.start = self.current;
    }
    fn advance(&mut self) -> &str {
        self.current+=1;
        &self.source[self.start..self.current]
    }
    
    fn retreat(&mut self) -> &str {
        self.current-=1;
        &self.source[self.start..self.current]
    }
    fn peek(&self) -> char {
        self.peek_offset(0)
    }

    fn peek_offset(&self, offset: usize) -> char {
        if let Some(ch) = self.source
            .chars()
            .nth(self.current + offset) {
                ch
            }else{
                '\0'
            }
    }

    fn scan_tokens(&mut self) -> Result<&Vec<Token>, &Vec<LanguageError>> {

        while self.current < self.source.len() {

            match self.advance() {
                "(" => self.add_token(TokenType::LEFTPAREN),
                ")" => self.add_token(TokenType::RIGHTPAREN),
                "{" => self.add_token(TokenType::LEFTBRACE),
                "}" => self.add_token(TokenType::RIGHTBRACE),
                "," => self.add_token(TokenType::COMMA),
                "." => self.add_token(TokenType::DOT),
                "-" => self.add_token(TokenType::MINUS),
                "+" => self.add_token(TokenType::PLUS),
                ";" => self.add_token(TokenType::SEMICOLON),
                "*" => self.add_token(TokenType::STAR),
                "!" => {
                        if self.advance() == "!=" {
                            self.add_token(TokenType::BANGEQUAL);
                        } else {
                            self.retreat();
                            self.add_token(TokenType::BANG);
                        }
                },
                "=" => {
                        if self.advance() == "==" {
                            self.add_token(TokenType::EQUALEQUAL);
                        } else {
                            self.retreat();
                            self.add_token(TokenType::EQUAL);
                        }
                },
                ">" => {
                        if self.advance() == ">=" {
                            self.add_token(TokenType::GREATEREQUAL);
                        } else {
                            self.retreat();
                            self.add_token(TokenType::GREATER);
                        }
                }
                "<" => {
                        if self.advance() == "<=" {
                            self.add_token(TokenType::LESSEQUAL);
                        } else {
                            self.retreat();
                            self.add_token(TokenType::LESS);
                        }
                }
                "/" => {
                        if self.advance() == "//" {
                            //comment
                            while self.advance() != "\n" 
                                   && self.current < self.source.len() {
                            }
                               self.start = self.current;
                        }else{
                            self.retreat();
                            self.add_token(TokenType::SLASH);
                        }
                }
                " " | "\r" | "\t" => self.start = self.current,
                "\n" => {
                    self.start = self.current;
                    self.line += 1;
                }
                "\"" => {
                    
                    match self.seek_string() {
                        Ok(s) => self.add_token(TokenType::STRING(s)),
                        Err(e) => self.errors.push(e)
                    }

                }
                "0" | "1" | "2" | 
                "3" | "4" | "5" | 
                "6" | "7" | "8" |
                "9" => { 
                  match self.seek_number() {
                      Ok(n) => self.add_token(TokenType::NUMBER(n)),
                      Err(e) => self.errors.push(e)
                  };
                },
                chs if is_alpha(chs) => {
                    while self.peek().is_alphanumeric() || self.peek() == '_' {
                       self.advance(); 
                    }

                    self.keyword_or_identifier()
                    
                },
                e => eprintln!("Unexpected token {:?}", e)

            }
        }

        self.add_token(TokenType::EOF);

        //check for syntax errors
        if !self.has_errors() {
            Ok(&self.tokens)
        }else{
            Err(&self.errors)
        }
    }

    fn keyword_or_identifier(&mut self) {
        
        self.add_token(
            match &self.source[self.start..self.current] {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "fun" => TokenType::FUN,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            s => TokenType::IDENTIFIER(s.into())
        });
    }

    fn seek_number(&mut self) -> Result<f64, LanguageError> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' &&
            is_digit(self.peek_offset(1 as usize)) {
            self.advance();

            while is_digit(self.peek()){self.advance();}
        }

        match self.source[self.start..self.current].parse::<f64>() {
            Ok(num) => Ok(num),
            Err(err) => Err(LanguageError::SyntaxError(format!("{}", err)))
        }
    }

    fn seek_string(&mut self) -> Result<String, LanguageError> {
        while self.peek() != '"' &&
                self.current < self.source.len() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.current == self.source.len() {
            Err(LanguageError::SyntaxError("Unterminated String".into()))
        }else{
            self.advance();
            Ok(self.source[self.start+1..self.current-1].into())
        }
    }


    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

fn is_alpha(chs: &str) -> bool {
        if let Some(ch) = chs.chars().next() {
            ch.is_alphabetic()
        }else{
            false
        }
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn run(source: &String) -> Result<(), &'static str> {

    let mut scanner: Scanner = Scanner::new(source.clone());
    
    match scanner.scan_tokens() {
        Ok(tokens) => {
             for token in tokens.iter() {
                println!("{:?}", token);
            }
            Ok(())
        },
        Err(errs) => {
            for err in errs.iter() {
                eprintln!("{:?}", err);
            }
            Err("Execution Error")
        }
    }
     
}

fn run_file(file: &String){
    let contents = fs::read_to_string(file)
                    .expect("Error:: Reading the file");

    run(&contents).expect("Execution Error");
}

fn run_prompt(){
    
    let mut lock = io::stdout().lock();

    loop {
        write!(lock, ">").unwrap();
        io::stdout().flush().unwrap();

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)
                    .expect("Error reading stdin");

        run(&cmd).unwrap_or(());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    }else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
