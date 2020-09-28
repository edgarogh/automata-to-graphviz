#[derive(Debug, Default)]
struct Automata {
  id_counter: usize,
  nodes: Vec<Node>,
  connections: Vec<Connection>,
}

impl Automata {
  fn gen_id(&mut self) -> usize {
    self.id_counter += 1;
    self.id_counter
  }
  
  fn get_node(&self, id: usize) -> &Node {
    self.nodes.iter().find(|node| node.id == id).unwrap()
  }
  
  fn push_or_merge_node(&mut self, mut node: Node) -> Result<usize, ()> {
    if let Some(old_node) = self.nodes.iter_mut().find(|old_node| old_node.name == node.name) {
      if old_node.acceptor != node.acceptor { return Err(()) }; // TODO describe
      Ok(old_node.id)
    } else {
      let id = self.gen_id();
      node.id = id;
      self.nodes.push(node);
      Ok(id)
    }
  }
  
  fn push_or_merge_connection(&mut self, mut conn: Connection) -> Result<usize, ()> {
    if let Some(old_conn) = self.connections.iter_mut().find(|old_conn| (old_conn.from, old_conn.to) == (conn.from, conn.to)) {
      old_conn.symbols.append(&mut conn.symbols);
      Ok(old_conn.id)
    } else {
      let id = self.gen_id();
      conn.id = id;
      self.connections.push(conn);
      Ok(id)
    }
  }
  
  fn defragment_ids(&mut self) {
    use std::collections::HashMap;
    
    let nodes = {
      let ids: HashMap<_, _> = self.nodes.iter().map(|node| node.id).enumerate().map(|(a, b)| (b, a)).collect();
      self.nodes.iter_mut().for_each(|node| node.id = ids[&node.id]);
      self.connections.iter_mut().for_each(|conn| {
        conn.from = ids[&conn.from];
        conn.to = ids[&conn.to];
      });
      ids.len()
    };
    
    let connections = {
      let ids: HashMap<_, _> = self.connections.iter().map(|conn| conn.id).enumerate().map(|(a, b)| (b, a)).collect();
      self.connections.iter_mut().for_each(|conn| conn.id = ids[&conn.id]);
      ids.len()
    };
    
    self.id_counter = std::cmp::max(nodes, connections);
  }
  
  fn write_graphviz(&self, mut w: impl std::fmt::Write) {
    let acceptors = self.nodes.iter().filter(|node| node.acceptor).map(|node| node.name.as_str());
    
    writeln!(&mut w, "digraph {{");
    writeln!(&mut w, "    rankdir = LR");
    writeln!(&mut w, "    splines = line");
    writeln!(&mut w, "    n0 [label=\"\",shape=none,height=.0,width=.0]");
    writeln!(&mut w, "    node [shape=\"doublecircle\"]");
    for a in acceptors {
      writeln!(&mut w, "    \"{}\"", a);
    }
    writeln!(&mut w, "    node [shape=\"circle\"]");
    writeln!(&mut w, "    n0 -> \"{}\"", self.nodes[0].name);
    for conn in self.connections.iter() {
      writeln!(&mut w,
        "    \"{}\" -> \"{}\" [label=\"{}\"]",
        self.get_node(conn.from).name,
        self.get_node(conn.to).name,
        conn.symbols.join(","),
      );
    }
    writeln!(&mut w, "}}");
  }
}

#[derive(Debug)]
struct Node {
  id: usize,
  name: String,
  acceptor: bool,
}

#[derive(Debug)]
struct Connection {
  id: usize,
  from: usize,
  to: usize,
  symbols: Vec<Box<str>>,
}

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
  fn from_str(str: &'a str) -> Result<Self, String> {
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

fn main() {
  let s = std::fs::read_to_string("/dev/stdin").unwrap();
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
          let id = automata.push_or_merge_node(Node { id: 0, name: node.name.into(), acceptor: node.acceptor }).unwrap();
          
          if prev_node.is_some() && prev_arrow_symbols.is_some() {
            automata.push_or_merge_connection(Connection {
              id: 0,
              from: prev_node.take().unwrap(),
              to: id,
              symbols: prev_arrow_symbols.take().unwrap(),
            });
          }
          
          if prev_start {
            let last_idx = automata.nodes.len() - 1;
            automata.nodes.swap(0, last_idx);
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
  
  println!("{}", {
    let mut gv = String::with_capacity(100);
    automata.write_graphviz(&mut gv);
    gv
  });
}
