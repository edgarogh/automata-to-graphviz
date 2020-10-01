#[derive(Debug, Default)]
pub struct Automata {
  id_counter: usize,
  nodes: Vec<Node>,
  connections: Vec<Connection>,
}

impl Automata {
  fn gen_id(&mut self) -> usize {
    self.id_counter += 1;
    self.id_counter
  }

  pub fn nodes(&self) -> &Vec<Node> {
    &self.nodes
  }

  pub fn connections(&self) -> &Vec<Connection> {
    &self.connections
  }
  
  pub fn get_node(&self, id: usize) -> &Node {
    self.nodes.iter().find(|node| node.id == id).unwrap()
  }
  
  pub fn push_or_merge_node(&mut self, mut node: Node) -> Result<usize, ()> {
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
  
  pub fn push_or_merge_connection(&mut self, mut conn: Connection) -> Result<usize, ()> {
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

  pub fn set_input(&mut self, id: usize) {
    self.nodes.swap(0, id);
  }
  
  pub fn defragment_ids(&mut self) {
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
  
  pub fn write_graphviz(&self, mut w: impl std::fmt::Write) {
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
pub struct Node {
  pub id: usize,
  pub name: String,
  pub acceptor: bool,
}

#[derive(Debug)]
pub struct Connection {
  pub id: usize,
  pub from: usize,
  pub to: usize,
  pub symbols: Vec<Box<str>>,
}