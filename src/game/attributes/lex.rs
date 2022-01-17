#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseArgumentItem {
  Str(String),
  Number(i64),
  Bool(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseItem {
  Key(String),
  Argument(ParseArgumentItem),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexItem {
  Paren,
  Comma,
  Space,
  Minus,
  Number(u8),
  Character(char),
}

fn lex(input: &String) -> Vec<LexItem> {
  input
    .chars()
    .into_iter()
    .map(|c| match c {
      '-' => LexItem::Minus,
      ' ' => LexItem::Space,
      ',' => LexItem::Comma,
      '(' | ')' => LexItem::Paren,
      '0'..='9' => LexItem::Number(c.to_string().parse::<u8>().unwrap()),
      c => LexItem::Character(c),
    })
    .collect()
}

fn parse_string(input: &Vec<LexItem>, current_pos: usize) -> Option<(usize, String)> {
  let mut pos = current_pos;
  let mut char_list = Vec::new();

  while let Some(item) = input.get(pos) {
    if let LexItem::Character(n) = item {
      char_list.push(n.to_string());
      pos += 1;
    } else if item == &LexItem::Minus {
      char_list.push('-'.to_string());
      pos += 1;
    } else if let LexItem::Number(n) = item {
      char_list.push(n.to_string());
      pos += 1;
    } else {
      break;
    }
  }

  Some((pos, char_list.join("")))
}

fn parse_number(input: &Vec<LexItem>, current_pos: usize) -> Option<(usize, i64)> {
  let mut pos = current_pos;
  let mut number_list = Vec::new();

  while let Some(item) = input.get(pos) {
    if let LexItem::Number(n) = item {
      number_list.push(n.to_string());
      pos += 1;
    } else {
      break;
    }
  }

  if let Ok(number) = number_list.join("").parse::<i64>() {
    Some((pos, number))
  } else {
    None
  }
}

fn parse(input: Vec<LexItem>) -> Result<Vec<ParseItem>, String> {
  let mut result = Vec::new();
  let mut pos = 0;
  let mut within_arguments = false;
  while let Some(item) = input.get(pos) {
    match item {
      LexItem::Paren => {
        pos += 1;
        within_arguments = true;
      },
      LexItem::Comma => {
        pos += 1;
      },
      LexItem::Space => {
        pos += 1;
      },
      // Negative Number
      LexItem::Minus => {
        if let Some((new_pos, n)) = parse_number(&input, pos + 1) {
          pos = new_pos;
          result.push(ParseItem::Argument(ParseArgumentItem::Number(-n)));
        } else {
          return Err(format!("Failed to parse Minus at pos {}", pos));
        }
      },
      LexItem::Number(_) => {
        if let Some((new_pos, n)) = parse_number(&input, pos) {
          pos = new_pos;
          result.push(ParseItem::Argument(ParseArgumentItem::Number(n)));
        }
      },
      LexItem::Character(_) => {
        if let Some((new_pos, item)) = parse_string(&input, pos) {
          pos = new_pos;
          if let Ok(b) = item.parse::<bool>() {
            result.push(ParseItem::Argument(ParseArgumentItem::Bool(b)));
          } else if within_arguments {
            result.push(ParseItem::Argument(ParseArgumentItem::Str(item)));
          } else {
            result.push(ParseItem::Key(item));
          }
        }
      },
    }
  }

  Ok(result)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeEntry(pub String, pub Vec<ParseArgumentItem>);

impl From<String> for AttributeEntry {
  fn from(entry: String) -> Self {
    let tokens = lex(&entry);
    match parse(tokens) {
      Ok(items) => {
        if let Some(ParseItem::Key(key)) = items.get(0) {
          let args: Vec<_> = Vec::from_iter(items[1..].iter().cloned())
            .iter()
            .map(|x| {
              if let ParseItem::Argument(arg) = x {
                arg.clone()
              } else {
                panic!("Found second key in entry {}", entry);
              }
            })
            .collect();
          AttributeEntry(key.clone(), args)
        } else {
          panic!("Failed to find key in attribute {}", entry);
        }
      },
      Err(err) => panic!("{}", err),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::game::attributes::lex::*;

  #[test]
  fn test_lex() {
    assert_eq!(
      lex(&"d(0,9)".to_string()),
      vec![
        LexItem::Character('d'),
        LexItem::Paren,
        LexItem::Number(0),
        LexItem::Comma,
        LexItem::Number(9),
        LexItem::Paren
      ]
    );
    assert_eq!(
      lex(&"d(1, 2)".to_string()),
      vec![
        LexItem::Character('d'),
        LexItem::Paren,
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Space,
        LexItem::Number(2),
        LexItem::Paren
      ]
    );
    assert_eq!(
      lex(&"d(-1, 2)".to_string()),
      vec![
        LexItem::Character('d'),
        LexItem::Paren,
        LexItem::Minus,
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Space,
        LexItem::Number(2),
        LexItem::Paren
      ]
    );
  }

  #[test]
  fn test_parse_number() {
    assert_eq!(
      parse_number(&vec![LexItem::Number(1), LexItem::Number(2)], 0),
      Some((2, 12))
    );
    assert_eq!(
      parse_number(&vec![LexItem::Number(1), LexItem::Number(2), LexItem::Paren], 0),
      Some((2, 12))
    );
    assert_eq!(
      parse_number(&vec![LexItem::Number(1), LexItem::Paren, LexItem::Number(2)], 0),
      Some((1, 1))
    );
  }

  #[test]
  fn test_parse_string() {
    assert_eq!(
      parse_string(&vec![LexItem::Character('a'), LexItem::Character('b')], 0),
      Some((2, "ab".to_string()))
    );
    assert_eq!(
      parse_string(
        &vec![LexItem::Character('a'), LexItem::Paren, LexItem::Character('b')],
        0
      ),
      Some((1, "a".to_string()))
    );
  }

  #[test]
  fn test_parse() {
    assert_eq!(
      parse(vec![
        LexItem::Character('a'),
        LexItem::Paren,
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Number(2),
        LexItem::Paren
      ]),
      Ok(vec![
        ParseItem::Key("a".to_string()),
        ParseItem::Argument(ParseArgumentItem::Number(1)),
        ParseItem::Argument(ParseArgumentItem::Number(2))
      ])
    );

    assert_eq!(
      parse(vec![
        LexItem::Character('a'),
        LexItem::Paren,
        LexItem::Number(1),
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Number(2),
        LexItem::Paren
      ]),
      Ok(vec![
        ParseItem::Key("a".to_string()),
        ParseItem::Argument(ParseArgumentItem::Number(11)),
        ParseItem::Argument(ParseArgumentItem::Number(2))
      ])
    );

    assert_eq!(
      parse(vec![
        LexItem::Character('a'),
        LexItem::Paren,
        LexItem::Number(1),
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Character('b'),
        LexItem::Character('a'),
        LexItem::Paren
      ]),
      Ok(vec![
        ParseItem::Key("a".to_string()),
        ParseItem::Argument(ParseArgumentItem::Number(11)),
        ParseItem::Argument(ParseArgumentItem::Str("ba".to_string()))
      ])
    );

    assert_eq!(
      parse(vec![
        LexItem::Character('a'),
        LexItem::Paren,
        LexItem::Minus,
        LexItem::Number(1),
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Character('t'),
        LexItem::Character('r'),
        LexItem::Character('u'),
        LexItem::Character('e'),
        LexItem::Paren
      ]),
      Ok(vec![
        ParseItem::Key("a".to_string()),
        ParseItem::Argument(ParseArgumentItem::Number(-11)),
        ParseItem::Argument(ParseArgumentItem::Bool(true))
      ])
    );

    assert_eq!(
      parse(vec![
        LexItem::Character('a'),
        LexItem::Paren,
        LexItem::Minus,
        LexItem::Number(1),
        LexItem::Number(1),
        LexItem::Comma,
        LexItem::Character('t'),
        LexItem::Character('r'),
        LexItem::Minus,
        LexItem::Character('3'),
        LexItem::Paren
      ]),
      Ok(vec![
        ParseItem::Key("a".to_string()),
        ParseItem::Argument(ParseArgumentItem::Number(-11)),
        ParseItem::Argument(ParseArgumentItem::Str("tr-3".to_string()))
      ])
    );

    assert_eq!(
      parse(vec![LexItem::Character('a')]),
      Ok(vec![ParseItem::Key("a".to_string())])
    );
  }

  #[test]
  fn test_parse_integration() {
    assert_eq!(
      AttributeEntry::from("d(1,2,test,true)".to_string()),
      AttributeEntry(
        "d".to_string(),
        vec![
          ParseArgumentItem::Number(1),
          ParseArgumentItem::Number(2),
          ParseArgumentItem::Str("test".to_string()),
          ParseArgumentItem::Bool(true)
        ]
      )
    );
    assert_eq!(
      AttributeEntry::from("d".to_string()),
      AttributeEntry("d".to_string(), vec![])
    );
    assert_eq!(
      AttributeEntry::from("d()".to_string()),
      AttributeEntry("d".to_string(), vec![])
    );
  }
}
