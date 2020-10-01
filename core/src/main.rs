mod eoa;
mod models;

fn main() {
  let s = std::fs::read_to_string("/dev/stdin").unwrap();
  let automata = eoa::parse(&s);
  
  println!("{}", {
    let mut gv = String::with_capacity(100);
    automata.write_graphviz(&mut gv);
    gv
  });
}