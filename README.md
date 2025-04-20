adg_lang/
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── parser.rs
│   ├── interpreter.rs
│   ├── runtime/
│   │   ├── math.rs
│   │   ├── fs.rs
│   │   ├── net.rs
│   │   └── web.rs
├── examples/
│   └── hello.adg
├── Cargo.toml

#### ✅ Controle de fluxo
- [ ] `else` nos blocos `if`
- [ ] `while`, `for`, `break`, `continue`

#### ✅ Definição de funções
- [X] Suporte a `fn nome(args) { ... }`
- [X] Suporte a `return` (giveback)

#### ✅ Recursos nativos
- [ ] Módulo `math` com `sqrt`, `pow`, `random`
- [ ] Módulo `fs` com `readFile`, `writeFile`
- [ ] Módulo `net` com `fetch`
- [ ] Módulo `web` com `route`, `listen` (tipo Fastify)

#### ✅ Execução real de `.adg` via terminal
- [ ] `cargo run arquivo.adg`