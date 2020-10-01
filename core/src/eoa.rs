use crate::models::*;

#[derive(Debug)]
struct TokenNode<'a> {
  name: &'a str,
  acceptor: bool,
  stay: Box<[&'a str]>,
}

#[derive(Debug)]
enum Token<'a> {
  Arrow(Option<Box<[&'a str]>>),
  Node(TokenNode<'a>),
}

impl<'a> Token<'a> {
  pub fn from_str(str: &'a str) -> Result<Self, String> {
    let first_an = str.chars().next().unwrap().is_alphanumeric();    

    if str == "->" {
      Ok(Self::Arrow(None))
    } else if str.starts_with('-') && str.ends_with('>') {
      Ok(Self::Arrow(Some(str[1..(str.len()-1)].split(',').collect())))
    } else if str.starts_with('(') {
      let end = str.find(')').ok_or(String::from("Missing closing brace"))?;
      let stay = if end == str.len() - 1 {
        Default::default()
      } else if let Some("^") = str.get((end+1)..=(end+1)) {
        str[(end+2)..].split(',').collect()
      } else {
        return Err("Unknown expression after `)`".into());
      };

      Ok(Self::Node(TokenNode { name: &str[1..end], acceptor: true, stay }))
    } else if first_an {
      let (name, stay) = str
        .find('^')
        .map(|pos| (&str[..pos], str[(pos+1)..].split(',').collect()))
        .unwrap_or((str, Default::default()));

      Ok(Self::Node(TokenNode { name, acceptor: false, stay }))
    } else {
      Err("unknown token".into())
    }
  }
}

pub fn parse(s: &str) -> Automata {
  let lines = s.lines();
  
  let mut automata = Automata::default();
  let mut first_found = false;

  for line in lines {
    let tokens: Result<Vec<_>, _> = line.split_whitespace().map(Token::from_str).collect();
    
    let mut prev_start = false;
    let mut prev_node: Option<usize> = None;
    let mut prev_arrow_symbols: Option<Vec<Box<str>>> = None;

    for token in tokens.unwrap().into_iter() {
      match token {
        Token::Arrow(None) => {
          if first_found { panic!("Found an unexpected second start arrow") }; // TODO allow mid-line
          first_found = true;
          prev_start = true;
        }
        Token::Arrow(Some(slice)) => {
          if prev_node.is_some() {
            if !prev_start {
              prev_arrow_symbols = Some(slice.into_iter().copied().map(Box::from).collect());
            } else {
              panic!("Unexpected connection arrow behind a start arrow");
            }
          } else {
            panic!("Unexpected connection arrow behind nothing");
          }
        }
        Token::Node(node) => {
          let id = automata.push_or_merge_node(Node {
            id: 0,
            name: node.name.into(),
            acceptor: node.acceptor,
          }).unwrap();
          
          if prev_node.is_some() && prev_arrow_symbols.is_some() {
            automata.push_or_merge_connection(Connection {
              id: 0,
              from: prev_node.take().unwrap(),
              to: id,
              symbols: prev_arrow_symbols.take().unwrap(),
            });
          }
          
          if prev_start {
            let last_idx = automata.nodes().len() - 1;
            automata.set_input(last_idx);
          }
          
          prev_start = false;
          
          if node.stay.len() > 0 {
            automata.push_or_merge_connection(Connection {
              id: 0,
              from: id,
              to: id,
              symbols: node.stay.into_iter().copied().map(Box::from).collect(),
            });
          }
          
          prev_node = Some(id);
        }
      }
    }
  }

  automata
}

#[cfg(test)]
mod tests {
  #[test]
  fn tokenize() {

  }
}
