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

Perfeito! Isso quer dizer que a **execução está 100% funcional**! 🟢  
Nenhum erro, nenhuma exceção, e o comportamento está **correto**: como `idade` é `17`, o `if (idade >= 18)` **não entrou**, então não teve nenhum `print`.

---

### 🔍 Pra ver o print funcionando

Tenta mudar o valor para algo assim:

```adg
const idade = 20;
```

A saída será:

```
🧠 Execução:
maior de idade
```

---

### 🎯 Próximos passos sugeridos

Agora que o **core da linguagem ADG está funcional**, você pode começar a evoluir com:

#### ✅ Controle de fluxo
- [ ] `else` nos blocos `if`
- [ ] `while`, `for`, `break`, `continue`

#### ✅ Definição de funções
- [ ] Suporte a `fn nome(args) { ... }`
- [ ] Suporte a `return`

#### ✅ Recursos nativos
- [ ] Módulo `math` com `sqrt`, `pow`, `random`
- [ ] Módulo `fs` com `readFile`, `writeFile`
- [ ] Módulo `net` com `fetch`
- [ ] Módulo `web` com `route`, `listen` (tipo Fastify)

#### ✅ Execução real de `.adg` via terminal
- [ ] `cargo run arquivo.adg`

---

Me diz o que você quer priorizar agora que a base está sólida.  
Podemos seguir por fluxo (`if/else/while`), ou já partir para módulos nativos como `fs`, `math`, `web` ou até um compilador para `.adgbc`. Só falar!