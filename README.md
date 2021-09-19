## A Rust implementation of Lox
### Lox Language from *Crafting Interpreters* by Robert Nystrom

## Execution Modes
1. Source file input
```bash
$ cargo run -- "test/test_content/test_file.lox"
Ok(Token { token_type: LeftParen, lexeme: "(", line: 1 })
Ok(Token { token_type: Number(1.1), lexeme: "1.1", line: 1 })
Ok(Token { token_type: Minus, lexeme: "-", line: 1 })
Ok(Token { token_type: Number(2.2), lexeme: "2.2", line: 1 })
Ok(Token { token_type: RightParen, lexeme: ")", line: 1 })
(group (1.1 Minus 2.2))
$
```
2. REPL
```bash
$ cargo run
rlox] 1.1 + 2.2
Ok(Token { token_type: Number(1.1), lexeme: "1.1", line: 1 })
Ok(Token { token_type: Plus, lexeme: "+", line: 1 })
Ok(Token { token_type: Number(2.2), lexeme: "2.2", line: 1 })
(1.1 Plus 2.2)
OK.
rlox] quit
ok...see ya!
$ 
```