use super::lexer::{Lexeme, Token};
use super::Lexer;
use super::OP_CODES;

pub trait Statement {}

pub struct MixStatement {
  symbol: Option<Symbol>,
  op: String,
  a_part: Option<Box<Node>>,
  index_part: Option<Box<Node>>,
  f_part: Option<Box<Node>>,
}

impl Statement for MixStatement {}

pub struct EquStatement {
  symbol: Option<Symbol>,
  address: Box<Node>,
}

impl Statement for EquStatement {}

pub struct OrigStatement {
  symbol: Option<Symbol>,
  address: Box<Node>,
}

impl Statement for OrigStatement {}

pub struct ConStatement {
  symbol: Option<Symbol>,
  address: Box<Node>,
}

impl Statement for ConStatement {}

pub struct AlfStatement {
  symbol: Option<Symbol>,
  char_code: String,
}

impl Statement for AlfStatement {}

pub struct EndStatement {
  symbol: Option<Symbol>,
  address: Box<Node>,
}

impl Statement for EndStatement {}

pub struct Program {
  statements: Vec<Box<Statement>>,
}

pub trait Node {}

pub struct Number {
  value: usize,
}

impl Node for Number {}

pub struct Asterisk {}

impl Node for Asterisk {}

pub struct Symbol {
  name: String,
}

impl Node for Symbol {}

pub struct LiteralConstant {
  value: Box<Node>,
}

impl Node for LiteralConstant {}

pub struct Expression {
  left: Option<Box<Node>>,
  operator: Token,
  right: Box<Node>,
}

impl Node for Expression {}

pub struct WValue {
  parts: Vec<WValuePart>,
}

pub struct WValuePart {
  expression: Box<Node>,
  f_part: Option<Box<Node>>,
}

impl Node for WValue {}

struct ParseBuffer {
  scanned_values: Vec<Lexeme>,
  unscanned_values: Vec<Lexeme>,
}

pub struct Parser {
  lexer: Lexer,
  buffer: ParseBuffer,
}

impl Parser {
  pub fn new(input: &String) -> Parser {
    Parser {
      lexer: Lexer::new(input),
      buffer: ParseBuffer {
        scanned_values: vec![],
        unscanned_values: vec![],
      },
    }
  }

  pub fn parse<'a>(&mut self) -> Result<Program, &'a str> {
    let mut program = Program { statements: vec![] };

    loop {
      let lexeme = self.scan();
      if lexeme.token == Token::EOF {
        break;
      }
      self.unscan();
      let statement = self.parse_statement()?;
      program.statements.push(statement);
    }

    Ok(program)
  }

  fn parse_statement<'a>(&mut self) -> Result<Box<dyn Statement>, &'a str> {
    let lexeme = self.scan();
    if lexeme.token == Token::EOL {
      return self.parse_statement();
    }
    self.unscan();

    let symbol = self.parse_symbol();
    let opcode = self.parse_opcode()?;

    let statement: Box<Statement> = match opcode.as_str() {
      "EQU" => Box::new(self.parse_equ_statement(symbol)?),
      "ORIG" => Box::new(self.parse_orig_statement(symbol)?),
      "CON" => Box::new(self.parse_con_statement(symbol)?),
      "ALF" => Box::new(self.parse_alf_statement(symbol)?),
      "END" => Box::new(self.parse_end_statement(symbol)?),
      _ => Box::new(self.parse_mix_statement(symbol, opcode)?),
    };

    Ok(statement)
  }

  fn parse_mix_statement<'a>(
    &mut self,
    symbol: Option<Symbol>,
    op: String,
  ) -> Result<MixStatement, &'a str> {
    if !OP_CODES.contains_key(&op[..]) {
      println!("opcode, {}", op);
      return Err("unknown op code");
    }

    self.swallow_whitespace();

    let a_part = self.parse_a_part()?;
    let index_part = self.parse_index_part()?;
    let f_part = self.parse_f_part()?;

    let lexeme = self.scan_ignore_whitespace();
    if lexeme.token != Token::EOL {
      return Err("expected EOL");
    }

    Ok(MixStatement {
      symbol,
      op,
      a_part,
      index_part,
      f_part,
    })
  }

  fn parse_equ_statement<'a>(&mut self, symbol: Option<Symbol>) -> Result<EquStatement, &'a str> {
    self.swallow_whitespace();

    let w_value = self.parse_w_value()?;
    let address = match w_value {
      None => return Err("expected W-value"),
      Some(w_value) => w_value,
    };

    let lexeme = self.scan_ignore_whitespace();
    if lexeme.token != Token::EOL {
      return Err("expected EOL");
    }

    Ok(EquStatement {
      symbol,
      address: Box::new(address),
    })
  }

  fn parse_orig_statement<'a>(&mut self, symbol: Option<Symbol>) -> Result<OrigStatement, &'a str> {
    self.swallow_whitespace();

    let w_value = self.parse_w_value()?;
    let address = match w_value {
      None => return Err("expected W-value"),
      Some(w_value) => w_value,
    };

    let lexeme = self.scan_ignore_whitespace();
    if lexeme.token != Token::EOL {
      return Err("expected EOL");
    }

    Ok(OrigStatement {
      symbol,
      address: Box::new(address),
    })
  }

  fn parse_con_statement<'a>(&mut self, symbol: Option<Symbol>) -> Result<ConStatement, &'a str> {
    self.swallow_whitespace();

    let w_value = self.parse_w_value()?;
    let address = match w_value {
      None => return Err("expected W-value"),
      Some(w_value) => w_value,
    };

    let lexeme = self.scan_ignore_whitespace();
    if lexeme.token != Token::EOL {
      return Err("expected EOL");
    }

    Ok(ConStatement {
      symbol,
      address: Box::new(address),
    })
  }

  fn parse_alf_statement<'a>(&mut self, symbol: Option<Symbol>) -> Result<AlfStatement, &'a str> {
    self.swallow_whitespace();

    let lexeme = self.scan();
    if lexeme.token != Token::STRINGLITERAL {
      return Err("expected string literal");
    }

    let char_code = lexeme.literal;

    let lexeme = self.scan_ignore_whitespace();
    if lexeme.token != Token::EOL {
      return Err("expected EOL");
    }

    Ok(AlfStatement { symbol, char_code })
  }

  fn parse_end_statement<'a>(&mut self, symbol: Option<Symbol>) -> Result<EndStatement, &'a str> {
    self.swallow_whitespace();

    let w_value = self.parse_w_value()?;
    let address = match w_value {
      None => return Err("expected W-value"),
      Some(w_value) => w_value,
    };

    let lexeme = self.scan_ignore_whitespace();
    if lexeme.token != Token::EOL {
      return Err("expected EOL");
    }

    Ok(EndStatement {
      symbol,
      address: Box::new(address),
    })
  }

  fn parse_a_part(&mut self) -> Result<Option<Box<Node>>, &'static str> {
    if let Some(expression) = self.parse_expression()? {
      return Ok(Some(expression));
    }

    let quote = self.scan();
    if quote.token == Token::LITERALQUOTE {
      match self.parse_expression()? {
        None => Err("expected expression after literal quote"),
        Some(expression) => {
          let quote = self.scan();
          if quote.token != Token::LITERALQUOTE {
            return Err("expected closing literal quote");
          }
          Ok(Some(Box::new(LiteralConstant { value: expression })))
        }
      }
    } else {
      self.unscan();
      Ok(None)
    }
  }

  fn parse_index_part(&mut self) -> Result<Option<Box<Node>>, &'static str> {
    let comma = self.scan();
    if comma.token == Token::COMMA {
      match self.parse_expression()? {
        None => Err("expected expression after comma"),
        Some(expression) => Ok(Some(expression)),
      }
    } else {
      self.unscan();
      Ok(None)
    }
  }

  fn parse_f_part(&mut self) -> Result<Option<Box<Node>>, &'static str> {
    let lparen = self.scan();
    if lparen.token == Token::LPAREN {
      match self.parse_expression()? {
        None => Err("expected expression after lparen"),
        Some(expression) => {
          let rparen = self.scan();
          if rparen.token != Token::RPAREN {
            return Err("expected closing rparen");
          }
          Ok(Some(expression))
        }
      }
    } else {
      self.unscan();
      Ok(None)
    }
  }

  fn parse_w_value(&mut self) -> Result<Option<WValue>, &'static str> {
    let part = self.parse_w_value_part()?;
    let part = match part {
      None => return Ok(None),
      Some(part) => part,
    };

    let mut parts = vec![part];

    loop {
      let comma = self.scan();
      if comma.token != Token::COMMA {
        self.unscan();
        return Ok(Some(WValue { parts: parts }));
      }

      let next_part = self.parse_w_value_part()?;
      let next_part = match next_part {
        None => return Err("expected W-value part after comma"),
        Some(next_part) => next_part,
      };

      parts.push(next_part);
    }
  }

  fn parse_w_value_part(&mut self) -> Result<Option<WValuePart>, &'static str> {
    let expression = self.parse_expression()?;
    let expression = match expression {
      None => return Ok(None),
      Some(expression) => expression,
    };
    let f_part = self.parse_f_part()?;

    Ok(Some(WValuePart { expression, f_part }))
  }

  fn parse_expression(&mut self) -> Result<Option<Box<Node>>, &'static str> {
    let node: Box<Node> = match self.parse_atom() {
      Some(atom) => atom,
      None => {
        let lexeme = self.scan();
        if lexeme.token == Token::PLUS || lexeme.token == Token::MINUS {
          let atom = match self.parse_atom() {
            None => {
              self.unscan();
              return Err("Expected atom after [???]");
            }
            Some(atom) => atom,
          };

          Box::new(Expression {
            left: None,
            operator: lexeme.token,
            right: atom,
          })
        } else {
          self.unscan();
          return Ok(None);
        }
      }
    };

    self.parse_expression_tail(node)
  }

  fn parse_expression_tail(&mut self, head: Box<Node>) -> Result<Option<Box<Node>>, &'static str> {
    let lexeme = self.scan();
    match lexeme.token {
      Token::PLUS
      | Token::MINUS
      | Token::ASTERISK
      | Token::DIVIDE
      | Token::SHIFTDIVIDE
      | Token::FIELDSIGN => {
        let atom = match self.parse_atom() {
          None => {
            self.unscan();
            return Err("Expected atom after [???]");
          }
          Some(atom) => atom,
        };
        let expression = Box::new(Expression {
          left: Some(head),
          operator: lexeme.token,
          right: atom,
        });

        self.parse_expression_tail(expression)
      }
      _ => {
        self.unscan();
        Ok(Some(head))
      }
    }
  }

  fn parse_atom(&mut self) -> Option<Box<Node>> {
    let lexeme = self.scan();
    match lexeme.token {
      Token::NUMBER => {
        let value: usize = lexeme.literal.parse().unwrap();
        Some(Box::new(Number { value }))
      }
      Token::STRING => Some(Box::new(Symbol {
        name: lexeme.literal,
      })),
      Token::ASTERISK => Some(Box::new(Asterisk {})),
      _ => {
        self.unscan();
        None
      }
    }
  }

  fn parse_symbol(&mut self) -> Option<Symbol> {
    let lexeme = self.scan();
    if lexeme.token == Token::STRING {
      return Some(Symbol {
        name: lexeme.literal,
      });
    }

    self.unscan();
    None
  }

  fn parse_opcode(&mut self) -> Result<String, &'static str> {
    let lexeme = self.scan_ignore_whitespace();
    match lexeme.token {
      Token::STRING => Ok(lexeme.literal),
      _ => Err("expected op code"),
    }
  }

  fn swallow_whitespace(&mut self) {
    let lexeme = self.scan();
    if lexeme.token != Token::WS {
      self.unscan();
    }
  }

  fn scan_ignore_whitespace(&mut self) -> Lexeme {
    let lexeme = self.scan();
    if lexeme.token == Token::WS {
      self.scan()
    } else {
      lexeme
    }
  }

  fn scan(&mut self) -> Lexeme {
    let value = match self.buffer.unscanned_values.pop() {
      Some(val) => val,
      None => self.lexer.scan(),
    };

    self.buffer.scanned_values.push(value.clone());

    value
  }

  fn unscan(&mut self) {
    match self.buffer.scanned_values.pop() {
      Some(val) => self.buffer.unscanned_values.push(val),
      None => panic!("can't unscan"),
    }
  }
}
