# ADG Language

ADG (André Development Grammar) é uma linguagem de programação experimental interpretada, desenvolvida em Rust, com foco na simplicidade, modularidade e extensibilidade. Possui sintaxe semelhante ao JavaScript, suporte a funções, controle de fluxo, arrays, sistema de módulos e execução via CLI.

> 🚀 Projeto criado com fins educacionais e experimentais.

---

## ✨ Recursos da Linguagem

- ✅ Variáveis (`let` e `const`)
- ✅ Funções definidas pelo usuário
- ✅ Controle de fluxo: `if`, `else`, `looping`, `for`, `break`, `continue`
- ✅ Tipos primitivos: `number`, `string`, `boolean`, `null`, `array`
- ✅ Indexação de arrays (`x[0]`)
- ✅ Módulos nativos:
  - `math` → cálculos matemáticos
  - `style` → formatação com ANSI colors
  - `fb` (filebox) → leitura/escrita de arquivos

---

## 📦 Instalação

```bash
git clone https://github.com/euandrelucas/adg-language
cd adg-language
cargo build --release
```

---

## 🧪 Executando arquivos `.adg`

```bash
cargo run exemplo.adg
```

Ou diretamente:

```bash
./target/release/adg exemplo.adg
```

---

## 📄 Exemplo de Código

```adg
let nome = "André";
print(style.green("Olá, " + nome));

let numeros = [10, 20, 30];
print(numeros[1]); // imprime 20

fn dobro(x) {
    return x * 2;
}

print("Dobro: " + dobro(5));
```

---

## 📚 Módulos Nativos

### `math`
```adg
math.sqrt(25);       // → 5
math.pow(2, 3);       // → 8
math.random();        // → número aleatório
```

### `style` (cores e estilo no terminal)
```adg
print(style.red("Erro!"));
print(style.green("Tudo certo"));
print(style.bold("Importante"));
print(style.bgBlue("Azul fundo"));
```

### `fb` (filebox — arquivos)
```adg
let texto = fb.readFile("entrada.txt");
fb.writeFile("saida.txt", texto);
```

---

## 🛠 Estrutura Interna

- `main.rs` → CLI interpretador
- `lexer.rs` → tokenização
- `parser.rs` → geração da AST
- `interpreter.rs` → execução da AST
- `runtime/` → módulos nativos (`math`, `filebox`, `style`)

---

## 📌 Objetivos Futuros

- [ ] Suporte a objetos (`{ chave: valor }`)
- [ ] Importação de módulos (`import`)
- [ ] Sistema de tipos opcionais
- [ ] Interface de debugging

---

## 🤝 Contribuindo

1. Fork o repositório
2. Crie uma branch (`git checkout -b nova-funcionalidade`)
3. Commit suas alterações (`git commit -m 'Nova funcionalidade'`)
4. Push para a branch (`git push origin nova-funcionalidade`)
5. Abra um Pull Request

---

## 🧑‍💻 Autor

Desenvolvido por André Lucas – [andrepaiva.dev](https://andrepaiva.dev)

---

## 📄 Licença

Este projeto está licenciado sob a licença MIT.