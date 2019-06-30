use std::collections::HashMap;

//use crate::mix;

lazy_static! {
  static ref CHAR_CODES_TO_BYTES: HashMap<char, u8> = {
    let mut m = HashMap::new();
    m.insert(' ', 0);
	  m.insert('A', 1);
	  m.insert('B', 2);
	  m.insert('C', 3);
	  m.insert('D', 4);
	  m.insert('E', 5);
	  m.insert('F', 6);
	  m.insert('G', 7);
	  m.insert('H', 8);
	  m.insert('I', 9);
	  m.insert('∆', 10);
	  m.insert('J', 11);
	  m.insert('K', 12);
	  m.insert('L', 13);
	  m.insert('M', 14);
	  m.insert('N', 15);
	  m.insert('O', 16);
	  m.insert('P', 17);
	  m.insert('Q', 18);
	  m.insert('R', 19);
	  m.insert('∑', 20);
	  m.insert('∏', 21);
	  m.insert('S', 22);
	  m.insert('T', 23);
	  m.insert('U', 24);
	  m.insert('V', 25);
	  m.insert('W', 26);
	  m.insert('X', 27);
	  m.insert('Y', 28);
	  m.insert('Z', 29);
	  m.insert('0', 30);
	  m.insert('1', 31);
	  m.insert('2', 32);
	  m.insert('3', 33);
	  m.insert('4', 34);
	  m.insert('5', 35);
	  m.insert('6', 36);
	  m.insert('7', 37);
	  m.insert('8', 38);
	  m.insert('9', 39);
	  m.insert('.', 40);
	  m.insert(',', 41);
	  m.insert('(', 42);
	  m.insert(')', 43);
	  m.insert('+', 44);
	  m.insert('-', 45);
	  m.insert('*', 46);
	  m.insert('/', 47);
	  m.insert('=', 48);
	  m.insert('$', 49);
	  m.insert('<', 50);
	  m.insert('>', 51);
	  m.insert('@', 52);
	  m.insert(';', 53);
	  m.insert(':', 54);
	  m.insert('\'', 55);
    m
  };
}

pub fn get_code(char_code: &char) -> u8 {
  CHAR_CODES_TO_BYTES[char_code]
}
