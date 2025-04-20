# 📐 Módulo `math`

O módulo `math` da linguagem ADG fornece funções matemáticas utilitárias que podem ser usadas diretamente via `math.nomeDaFuncao(...)`.

> ✅ Todas as funções retornam um valor `number`.

---

## 📚 Índice de Funções

- [`math.sqrt(numero)`](#mathsqrtnumero)
- [`math.pow(base, expoente)`](#mathpowbase-expoente)
- [`math.random()`](#mathrandom)

---

## `math.sqrt(numero)`

Retorna a raiz quadrada do número informado.

### 📥 Parâmetros
- `numero` *(number)*: valor positivo ou zero

### 📤 Retorno
- A raiz quadrada como `number`.

### ✅ Exemplo
```adg
let raiz = math.sqrt(49);
print("Raiz: " + raiz); // Raiz: 7
```

---

## `math.pow(base, expoente)`

Retorna a potência de `base` elevado a `expoente`.

### 📥 Parâmetros
- `base` *(number)*: número base
- `expoente` *(number)*: número expoente

### 📤 Retorno
- O valor de `base^expoente`.

### ✅ Exemplo
```adg
let resultado = math.pow(2, 5);
print("Resultado: " + resultado); // Resultado: 32
```

---

## `math.random()`

Gera um número decimal aleatório entre `0` e `1`.

### 📥 Parâmetros
- Nenhum

### 📤 Retorno
- Um `number` aleatório entre `0` e `1`.

### ✅ Exemplo
```adg
let sorteio = math.random();
print("Número aleatório: " + sorteio);
```

---

## 🛠️ Observações Técnicas

- As funções `math` são implementadas em Rust via ponteiros para funções nativas (ver `runtime/math.rs`).
- São seguras, performáticas e retornam erros se usadas com argumentos incorretos.

---

## 📦 Importação

Não é necessário importar o módulo `math`. Ele já está disponível automaticamente no ambiente global com prefixo:

```adg
math.sqrt(...)
math.pow(...)
```

---

📁 [← Voltar para Documentação Principal](./docs/README.md)