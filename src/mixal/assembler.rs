use std::collections::HashMap;

use crate::mix;

use super::OP_CODES;

use super::{
  AlfStatement, Asterisk, ConStatement, EndStatement, EquStatement, Expression, LiteralConstant,
  MixStatement, Node, NodeVisitor, Number, OrigStatement, Program, Statement, StatementVisitor,
  Symbol, Token, WValue,
};

pub struct Assembler {
  pub words: HashMap<usize, mix::Word>,
  pub program_start: usize,
  location_counter: usize,
  symbol_table: HashMap<String, isize>,
  future_reference_table: HashMap<String, Vec<usize>>,
  literal_constant_table: HashMap<String, isize>,
}

impl Assembler {
  pub fn new() -> Assembler {
    Assembler {
      words: HashMap::new(),
      program_start: 0,
      location_counter: 0,
      symbol_table: HashMap::new(),
      future_reference_table: HashMap::new(),
      literal_constant_table: HashMap::new(),
    }
  }

  pub fn assemble(&mut self, program: Program) -> Result<(), &'static str> {
    for statement in program.statements.iter() {
      statement.accept(self)?;
    }
    Ok(())
  }

  fn get_value(&mut self, node: &dyn Node) -> isize {
    node.accept(self)
  }

  fn assemble_mix_statement(
    &mut self,
    statement: &MixStatement,
  ) -> Result<mix::Instruction, &'static str> {
    let op_info = match OP_CODES.get(&statement.op[..]) {
      None => return Err("unknown op code"),
      Some(op_info) => op_info,
    };

    let op_code = op_info.value;

    let address = match &statement.a_part {
      None => 0,
      Some(a) => self.get_value(a.as_ref()),
    };
    let address = mix::Address::from_value(address);

    let field_spec = match &statement.f_part {
      None => op_info.default_fs,
      Some(fp) => self.get_value(fp.as_ref()) as u8,
    };

    let index_part = match &statement.index_part {
      None => 0,
      Some(ip) => self.get_value(ip.as_ref()) as u8,
    };

    Ok(mix::Instruction {
      operation: op_code,
      address,
      modification: field_spec,
      index_specification: index_part,
    })
  }

  fn deal_with_symbol_declaration(&mut self, statement: &impl Statement) {
    let symbol = match statement.symbol() {
      None => return,
      Some(symbol) => symbol,
    };

    if symbol.is_local() {
      return;
    }

    self.add_symbol_here(symbol.internal_name())
  }

  fn deal_with_local_symbol_declaration(&mut self, statement: &impl Statement) {
    let symbol = match statement.symbol() {
      None => return,
      Some(symbol) => symbol,
    };

    if !symbol.is_local_declaration() {
      return;
    }

    self.add_symbol_here(symbol.internal_name())
  }

  fn add_symbol_here(&mut self, name: String) {
    self.add_symbol(name, self.location_counter as isize);
  }

  fn add_symbol(&mut self, name: String, value: isize) {
    eprintln!("adding symbol '{}' with value = {}", name, value);
    self.symbol_table.insert(name.clone(), value);
    self.fixup_future_refs(name);
  }

  fn insert_literal_constants(&mut self) {
    for (name, val) in self.literal_constant_table.clone() {
      let word = mix::Word::from_value(val);
      self.words.insert(self.location_counter, word);
      self.add_symbol_here(name.clone());
      self.location_counter += 1;
    }
  }

  fn add_future_ref(&mut self, name: String) {
    match self.future_reference_table.get_mut(&name) {
      None => {
        self
          .future_reference_table
          .insert(name, vec![self.location_counter]);
      }
      Some(refs) => {
        refs.push(self.location_counter);
      }
    };
  }

  fn fixup_future_refs(&mut self, name: String) {
    let refs = match self.future_reference_table.get(&name) {
      None => return,
      Some(refs) => refs,
    };
    let target = self.symbol_table[&name];

    for r in refs {
      let address = mix::Address::from_value(target);
      let mut word = self.words[r];
      word.sign = address.sign;
      word.bytes[0] = address.bytes[0];
      word.bytes[1] = address.bytes[1];
      self.words.insert(*r, word);
    }

    self.future_reference_table.remove(&name);
  }
}

impl StatementVisitor for Assembler {
  fn visit_mix_statement(&mut self, statement: &MixStatement) -> Result<(), &'static str> {
    self.deal_with_symbol_declaration(statement);
    let instruction = self.assemble_mix_statement(statement)?;
    let word = mix::Word::from_instruction(&instruction);
    self.words.insert(self.location_counter, word);
    self.deal_with_local_symbol_declaration(statement);
    self.location_counter += 1;

    Ok(())
  }

  fn visit_equ_statement(&mut self, statement: &EquStatement) -> Result<(), &'static str> {
    let name = statement.symbol.clone().unwrap().internal_name();
    let address = self.get_value(statement.address.as_ref());
    self.symbol_table.insert(name, address);

    Ok(())
  }

  fn visit_orig_statement(&mut self, statement: &OrigStatement) -> Result<(), &'static str> {
    self.deal_with_symbol_declaration(statement);
    let address = self.get_value(statement.address.as_ref());
    self.deal_with_local_symbol_declaration(statement);
    self.location_counter = address as usize;

    Ok(())
  }

  fn visit_con_statement(&mut self, statement: &ConStatement) -> Result<(), &'static str> {
    self.deal_with_symbol_declaration(statement);
    let address = self.get_value(statement.address.as_ref());
    let word = mix::Word::from_value(address);
    self.words.insert(self.location_counter, word);
    self.deal_with_local_symbol_declaration(statement);
    self.location_counter += 1;

    Ok(())
  }

  fn visit_alf_statement(&mut self, statement: &AlfStatement) -> Result<(), &'static str> {
    self.deal_with_symbol_declaration(statement);
    let char_code = &statement.char_code;
    let inner = &char_code[1..char_code.len() - 1];
    let word = mix::Word::from_char_code(inner);
    self.words.insert(self.location_counter, word);
    self.deal_with_local_symbol_declaration(statement);
    self.location_counter += 1;

    Ok(())
  }

  fn visit_end_statement(&mut self, statement: &EndStatement) -> Result<(), &'static str> {
    self.insert_literal_constants();

    self.deal_with_symbol_declaration(statement);
    let address = self.get_value(statement.address.as_ref());
    self.program_start = address as usize;
    self.deal_with_local_symbol_declaration(statement);

    Ok(())
  }
}

impl NodeVisitor for Assembler {
  fn visit_number(&mut self, number: &Number) -> isize {
    number.value as isize
  }

  fn visit_asterisk(&mut self, _: &Asterisk) -> isize {
    self.location_counter as isize
  }

  fn visit_symbol(&mut self, symbol: &Symbol) -> isize {
    if symbol.is_local_forward_ref() {
      self.add_future_ref(symbol.internal_name());
      return 0;
    }

    match self.symbol_table.get(&symbol.internal_name()) {
      None => {
        self.add_future_ref(symbol.internal_name());
        0
      }
      Some(value) => *value,
    }
  }

  fn visit_literal_constant(&mut self, literal_constant: &LiteralConstant) -> isize {
    let value = self.get_value(literal_constant.value.as_ref());
    let name = format!("__literal:{}", value);
    self.literal_constant_table.insert(name.clone(), value);
    self.add_future_ref(name);
    0
  }

  fn visit_expression(&mut self, expression: &Expression) -> isize {
    let left = match &expression.left {
      None => 0,
      Some(left) => self.get_value(left.as_ref()),
    };

    let right = self.get_value(expression.right.as_ref());

    match expression.operator {
      Token::PLUS => left + right,
      Token::MINUS => left - right,
      Token::ASTERISK => left * right,
      Token::DIVIDE => left / right,
      Token::SHIFTDIVIDE => panic!("the // operator is not implemented"),
      Token::FIELDSIGN => 8 * left + right,
      _ => panic!("unknown operator"),
    }
  }

  fn visit_w_value(&mut self, w_value: &WValue) -> isize {
    self.get_value(w_value.parts[0].expression.as_ref()) // TODO: make this work correctly
  }
}
