# 🎨 Módulo `style`

O módulo `style` da ADG Language permite aplicar **cores e estilos ANSI** a textos exibidos no terminal. Ideal para criar saídas mais visuais, alertas, logs coloridos e destaques durante a execução.

---

## 🎯 Como funciona

Todas as funções do módulo `style` recebem uma `string` e retornam essa string estilizada com códigos ANSI, já pronta para ser usada no `print`.

```adg
print(style.red("Erro"));
print(style.bold("Importante"));
```

---

## 📚 Funções disponíveis

### 🔵 Cores de texto

| Função             | Resultado no Terminal     |
|--------------------|---------------------------|
| `style.red(text)`      | 🔴 Vermelho                |
| `style.green(text)`    | 🟢 Verde                  |
| `style.yellow(text)`   | 🟡 Amarelo                |
| `style.blue(text)`     | 🔵 Azul                   |
| `style.magenta(text)`  | 🟣 Magenta                |
| `style.cyan(text)`     | 🔵 Ciano                  |
| `style.white(text)`    | ⚪ Branco                 |

---

### 🔲 Fundos coloridos

| Função               | Fundo ANSI              |
|----------------------|-------------------------|
| `style.bgRed(text)`     | 🔴 Fundo vermelho        |
| `style.bgGreen(text)`   | 🟢 Fundo verde           |
| `style.bgYellow(text)`  | 🟡 Fundo amarelo         |
| `style.bgBlue(text)`    | 🔵 Fundo azul            |

---

### ✍️ Estilos de texto

| Função               | Efeito                 |
|----------------------|------------------------|
| `style.bold(text)`       | Texto em **negrito**    |
| `style.underline(text)`  | Texto _sublinhado_      |

---

## ✅ Exemplos

```adg
print(style.red("Erro crítico!"));
print(style.green("Sucesso total"));
print(style.bold("Destaque"));
print(style.bgBlue("Fundo azul estiloso"));
```

Você também pode combinar com strings normais:

```adg
let nome = "André";
print(style.yellow("Bem-vindo, ") + nome);
```

---

## ⚠️ Notas

- O terminal precisa suportar **ANSI escape codes** (a maioria dos terminais modernos suporta).
- O módulo `style` é carregado automaticamente, não é necessário importar.

---

📁 [← Voltar para Documentação Principal](./docs/README.md)