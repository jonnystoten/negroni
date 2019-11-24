#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Token {
  ILLEGAL,
  EOF,
  EOL,
  WS,

  STRING,
  NUMBER,

  PLUS,
  MINUS,
  ASTERISK,
  DIVIDE,
  SHIFTDIVIDE,
  FIELDSIGN,

  COMMA,
  LPAREN,
  RPAREN,
  LITERALQUOTE,

  STRINGLITERAL,
  CHARCODE,
}

#[derive(Debug, Clone)]
pub struct Lexeme {
  pub token: Token,
  pub literal: String,
  pub line: usize,
  pub col: usize,
}

const EOF: char = 0 as char;

pub struct Lexer {
  r: Reader,
  line: usize,
  col: usize,
  last_cols: Vec<usize>,
}

impl Lexer {
  pub fn new(input: &String) -> Lexer {
    Lexer {
      r: Reader::new(input),
      line: 1,
      col: 0,
      last_cols: vec![],
    }
  }

  pub fn scan(&mut self) -> Lexeme {
    let (token, literal) = self.scan_token();
    Lexeme {
      token,
      literal,
      line: self.line,
      col: self.col,
    }
  }

  fn scan_token(&mut self) -> (Token, String) {
    let ch = self.read();

    if ch == '#' {
      // ignore everything up to the end of the line
      loop {
        let next = self.read();
        if next == '\n' {
          return (Token::EOL, next.to_string());
        }
      }
    }

    if ch == '*' && self.col == 1 {
      // ignore the whole line
      loop {
        let next = self.read();
        if next == '\n' {
          return self.scan_token();
        }
      }
    }

    if is_whitespace(ch) {
      self.unread();
      return self.scan_whitespace();
    }

    if is_alpha_num(ch) {
      self.unread();
      return self.scan_alpha_num();
    }

    if ch == '"' {
      return self.scan_string_literal();
    }

    if ch == EOF {
      return (Token::EOF, String::new());
    }

    let token = match ch {
      '\n' => Token::EOL,
      '+' => Token::PLUS,
      '-' => Token::MINUS,
      '*' => Token::ASTERISK,
      ':' => Token::FIELDSIGN,
      ',' => Token::COMMA,
      '(' => Token::LPAREN,
      ')' => Token::RPAREN,
      '=' => Token::LITERALQUOTE,
      _ => Token::ILLEGAL,
    };

    (token, ch.to_string())
  }

  fn scan_whitespace(&mut self) -> (Token, String) {
    loop {
      let ch = self.read();
      if ch == EOF {
        break;
      } else if !is_whitespace(ch) {
        self.unread();
        break;
      }
    }

    (Token::WS, " ".to_string())
  }

  fn scan_alpha_num(&mut self) -> (Token, String) {
    let mut buf = String::new();
    let mut all_digits = true;

    loop {
      let ch = self.read();
      if ch == EOF {
        break;
      } else if !is_alpha_num(ch) {
        self.unread();
        break;
      } else {
        if !is_digit(ch) {
          all_digits = false;
        }
        buf.push(ch);
      }
    }

    if all_digits {
      return (Token::NUMBER, buf);
    }

    return (Token::STRING, buf);
  }

  fn scan_string_literal(&mut self) -> (Token, String) {
    let mut buf = String::new();
    buf.push('"'); // initial quote

    loop {
      let ch = self.read();
      if ch == EOF {
        return (Token::ILLEGAL, buf);
      }

      buf.push(ch);

      if ch == '"' {
        return (Token::STRINGLITERAL, buf);
      }

      if !is_char_code(ch) {
        return (Token::ILLEGAL, buf);
      }
    }
  }

  fn read(&mut self) -> char {
    let ch = match self.r.read() {
      Some(c) => c,
      None => EOF,
    };

    if ch == '\n' {
      self.last_cols.push(self.col);
      self.line += 1;
      self.col = 0;
    } else {
      self.col += 1;
    }

    ch
  }

  fn unread(&mut self) {
    if self.col == 0 {
      self.line -= 1;
      self.col = self.last_cols.pop().unwrap();
    } else {
      self.col -= 1;
    }

    self.r.unread();
  }
}

struct Reader {
  chars: Vec<char>,
  read: Option<char>,
}

impl Reader {
  fn new(input: &String) -> Reader {
    let chars: Vec<char> = input.chars().rev().collect();
    Reader { chars, read: None }
  }

  fn read(&mut self) -> Option<char> {
    let result = self.chars.pop();
    self.read = result;
    result
  }

  fn unread(&mut self) -> () {
    let ch = match self.read {
      Some(ch) => ch,
      None => panic!("attempt to unread, but we've never been read"),
    };
    self.chars.push(ch);
    self.read = None;
  }
}

fn is_whitespace(ch: char) -> bool {
  ch == ' ' || ch == '\t'
}

fn is_letter(ch: char) -> bool {
  ('A' <= ch && ch <= 'Z') || ch == '∆' || ch == '∏' || ch == '∑'
}

fn is_digit(ch: char) -> bool {
  '0' <= ch && ch <= '9'
}

fn is_alpha_num(ch: char) -> bool {
  is_letter(ch) || is_digit(ch)
}

fn is_char_code(ch: char) -> bool {
  is_alpha_num(ch)
    || ch == ' '
    || ch == '.'
    || ch == ','
    || ch == '('
    || ch == ')'
    || ch == '+'
    || ch == '-'
    || ch == '*'
    || ch == '/'
    || ch == '='
    || ch == '$'
    || ch == '<'
    || ch == '>'
    || ch == '@'
    || ch == ';'
    || ch == ':'
    || ch == '\''
}
