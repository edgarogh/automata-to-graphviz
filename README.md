# DFA description DSL

## Examples:

`-> 1^b -a> (2)`

```
2 -b> -> (1) -a> 2
2 -a> P
(1) -b> P
```

`-> 1^a,b`

## Basic syntax

Nodes can be used without explicit declaration.

`1`, `P`: basic node
`(1)`, `(E)`: acceptor node
`->` before a node: input "connector" (only one allowed per DFA)
`-a>`, `-a,b>`: connectors, resp. for labels _{a}_, and _{a,b}_
`1^a`: looping connector (connector for symbol _a_, from _1_ to _1_) (shortcut)

<kbd>Enter</kbd>: new line = new expression with fresh context

## Usage

Unix stdin/stdout.

i.e: `cargo run < automata.txt`

## License

By Edgar Onghena <dev@edgar.bzh>
GPLv3
