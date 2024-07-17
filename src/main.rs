#[derive(Debug)]
struct Token {
    value: String,
    kind: String,
}

impl Token {
    fn new(value: String, kind: String) -> Token {
        Token { value, kind }
    }
}

struct Lexer {
    input: String,
    index: usize,
    end: bool,
    ch: char,
    tokens: Vec<Token>,
    keywords: Vec<String>,
    types: Vec<String>,
    arrows: Vec<String>,
    symbols: String,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let keywords = ["var", "val", "out", "in", "use"].map(String::from).to_vec();
        let types = ["any", "int", "bit", "str", "ray", "map"].map(String::from).to_vec();
        let arrows = ["<-", "->"].map(String::from).to_vec();
        Lexer { input: input.clone(), index: 0, end: false, ch: input.chars().next().unwrap(), tokens: Vec::new(), keywords, types, arrows, symbols: "*+&^|".to_string() }
    }

    fn next(&mut self) {
        self.index += 1;
        if self.index < self.input.len() {
            self.ch = self.input.chars().nth(self.index).unwrap();
        } else {
            self.end = true;
            self.ch = '\0';
        }
    }

    fn get_word(&mut self) -> String {
        let mut word = String::new();
        while !self.end && (self.ch.is_alphabetic() || self.ch == '_') {
            word.push(self.ch);
            self.next();
        }
        word
    }

    fn get_num(&mut self) -> String {
        let mut dp = false;
        let mut num = String::new();
        while !self.end && (self.ch.is_digit(10) || self.ch == '-' || self.ch == '.') {
            if self.ch == '.' {
                dp = true;
            }
            num.push(self.ch);
            self.next();
        }
        if dp {
            num.push('F');
        }
        num
    }

    fn get_string(&mut self) -> String {
        let mut string = String::new();
        while !self.end && self.ch != '"' {
            string.push(self.ch);
            self.next();
        }
        string
    }

    fn classify(&self, word: String) -> String {
        if self.keywords.contains(&word) {
            return "keyword".to_string();
        }
        if self.types.contains(&word) {
            return "type".to_string();
        }
        "name".to_string()
    }

    fn parse(&mut self) -> &Vec<Token> {
        while !self.end {
            if self.ch == '\n' || self.ch == ';' {
                self.tokens.push(Token::new("".to_string(), "".to_string()))
            }
            if self.ch.is_alphabetic() {
                let word = self.get_word();
                self.tokens.push(Token::new(word.clone(), self.classify(word)));
                continue; // prevent default
            }
            if self.ch == '<' || self.ch == '-' {
                let mut arrow = String::from(self.ch);
                arrow.push(self.input.chars().nth(self.index + 1).unwrap());
                if self.arrows.contains(&arrow) {
                    self.tokens.push(Token::new(arrow, "arrow".to_string()));
                    self.next();
                    self.next();
                    continue;
                }
            }
            if self.ch.is_digit(10) || self.ch == '-' {
                let num = self.get_num();
                let variant = if num.chars().last().unwrap() == 'F' { "num" } else { "int" };
                self.tokens.push(Token::new(num, variant.to_string()));
                continue; // prevent default
            }
            if self.ch == '"' {
                self.next();
                let string = self.get_string();
                self.tokens.push(Token::new(string, "str".to_string()));
            }
            if self.symbols.contains(self.ch) {
                self.tokens.push(Token::new(self.ch.to_string(), "flat".to_string()));
            }
            self.next(); // default iteration
        }
        &self.tokens
    }
}

fn main() {
    let input = "var int x\nx <- 0";
    let mut lexer = Lexer::new(input.to_string());
    lexer.parse();
    dbg!(lexer.tokens);
}
