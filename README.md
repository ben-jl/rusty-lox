## A Rust implementation of Lox
### Lox Language from *Crafting Interpreters* by Robert Nystrom

## Execution Modes
1. Source file input
```bash
$ cargo run -- rlox-cli/examples/example_p.lox 
22:39:35 [DEBUG] (1) rlox_parser: expression ["(Number 1.1 1:0)", "(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: unary      ["(Number 1.1 1:0)", "(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: primary    ["(Number 1.1 1:0)", "(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: factor     ["(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: term       ["(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: comparison ["(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: equality   ["(BangEqual 1:4)", "(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: unary      ["(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: primary    ["(Number 2.9 1:7)", "(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: factor     ["(Star 1:11)", "(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: unary      ["(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: primary    ["(Number 3.4 1:13)", "(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: term       ["(Eof 1:16)"]
22:39:35 [DEBUG] (1) rlox_parser: comparison ["(Eof 1:16)"]

1.10 BangEqual 2.90 Star 3.40
$
```
2. REPL
```bash
$ cargo run -- --debug
rlox] 1.1 + 2.2
22:33:07 [DEBUG] (1) rlox_parser: expression ["(Number 1.1 1:0)", "(Plus 1:4)", "(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: unary      ["(Number 1.1 1:0)", "(Plus 1:4)", "(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: primary    ["(Number 1.1 1:0)", "(Plus 1:4)", "(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: factor     ["(Plus 1:4)", "(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: term       ["(Plus 1:4)", "(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: unary      ["(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: primary    ["(Number 2.2 1:6)", "(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: factor     ["(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: comparison ["(Eof 1:9)"]
22:33:07 [DEBUG] (1) rlox_parser: equality   ["(Eof 1:9)"]

1.10 Plus 2.20
rlox] quit
Exiting...
$ 
```