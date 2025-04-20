# 📦 Módulo `fb` (filebox)

O módulo `fb` (apelido para `filebox`) permite **leitura e escrita de arquivos** diretamente pela linguagem ADG. Ele foi criado com o objetivo de ser simples, seguro e familiar para quem já conhece funções como `readFile` e `writeFile` em outras linguagens.

---

## 📚 Funções disponíveis

- [`fb.readFile(path)`](#fbreadfilepath)
- [`fb.writeFile(path, content)`](#fbwritefilepath-content)

---

## `fb.readFile(path)`

Lê o conteúdo de um arquivo e retorna como string.

### 📥 Parâmetros

- `path` *(string)*: caminho para o arquivo.

### 📤 Retorno

- Conteúdo do arquivo como string.

### ✅ Exemplo

```adg
let texto = fb.readFile("entrada.txt");
print("Conteúdo: " + texto);
```

---

## `fb.writeFile(path, content)`

Escreve o conteúdo em um arquivo. Se o arquivo não existir, ele será criado. Se já existir, será sobrescrito.

### 📥 Parâmetros

- `path` *(string)*: caminho do arquivo.
- `content` *(string)*: texto a ser escrito.

### 📤 Retorno

- `null` (sem retorno significativo).

### ✅ Exemplo

```adg
let mensagem = "Olá, mundo!";
fb.writeFile("saida.txt", mensagem);
```

---

## 🛠️ Observações Técnicas

- As funções são wrappers diretos para `std::fs` do Rust.
- Erros de leitura ou escrita (como caminho inválido) resultam em `panic` com mensagens detalhadas.
- O módulo é automaticamente carregado no interpretador como `fb`.

---

## 📦 Módulo Alternativo

Você também pode alterar o nome do módulo em versões futuras para algo como `arquivo`, `doc`, `vault`, etc. No momento, o padrão é:

```adg
fb.readFile(...)
fb.writeFile(...)
```

---

📁 [← Voltar para Documentação Principal](./docs/README.md)