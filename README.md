<h1 align="center">🧠 ADG Language</h1>

<p align="center">
  Uma linguagem de programação feita em Rust, simples, extensível e divertida de explorar.
</p>

<p align="center">
  <a href="https://github.com/euandrelucas/adg-language"><img src="https://img.shields.io/badge/github-euandrelucas/adg--language-8b5cf6?style=for-the-badge&logo=github"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-orange?style=for-the-badge&logo=rust"></a>
</p>

---

## 📌 Sobre

**ADG (André Development Grammar)** é uma linguagem interpretada criada do zero com Rust. Ela possui sintaxe semelhante ao JavaScript, com suporte a:

- Variáveis (`let`, `const`)
- Funções e expressões matemáticas
- Controle de fluxo (`if`, `looping`, `for`, `break`, `continue`)
- Arrays com indexação
- Execução via CLI (`.adg`)
- Módulos nativos: `math`, `style`, `fb` (filebox)

> Ideal para fins educacionais, experimentos com interpretadores, ou diversão geek.

---

## 🚀 Começando

### Clonar o projeto
```bash
git clone https://github.com/euandrelucas/adg-language.git
cd adg-language
```

### Build com Cargo

```bash
cargo build --release
```

---

## 🧪 Executando um arquivo `.adg`

```bash
cargo run exemplo.adg
# ou
./target/release/adg exemplo.adg
```

---

## 📄 Exemplo de código

```adg
print(style.green("Bem-vindo à ADG!"));

let x = [10, 20, 30];
print("Segundo número: " + x[1]);

fn somar(a, b) {
    return a + b;
}

print("Soma: " + somar(2, 3));
```

---

## 📚 Documentação

A documentação está disponível na pasta [`/docs`](./docs):

- [Introdução e sintaxe geral](./docs/README.md)
- [Módulo math](./docs/math.md)
- [Módulo style (cores ANSI)](./docs/style.md)
- [Módulo fb (filebox - arquivos)](./docs/fb.md)

---

## 📦 Estrutura do Projeto

```
📁 src/
├── main.rs           # CLI interpretador
├── lexer.rs          # Tokenizador
├── parser.rs         # Analisador sintático (AST)
├── interpreter.rs    # Executor da AST
└── runtime/
    ├── math.rs
    ├── filebox.rs
    └── style.rs
```

---

## 🧑‍💻 Autor

Desenvolvido com ❤️ por [André Lucas](https://github.com/euandrelucas)

---

## 🪪 Licença

Distribuído sob a licença MIT. Veja [`LICENSE`](./LICENSE) para mais informações.