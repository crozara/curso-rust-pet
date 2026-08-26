# 🦀 Curso: Desenvolvimento de Alta Performance com Rust e WebAssembly

Bem-vindo ao repositório do curso do Tech Talk! 

Este projeto contém a trilha prática desenvolvida para guiar o aprendizado desde a sintaxe básica e os fundamentos de memória (Ownership e Borrowing) do Rust, preparando o terreno para aplicações de alta performance.

---

## 👨‍🏫 Instrutores
* **Kauê Crozara da Silva**
* **João Pedro de Oliveira Leal**

---

## 🛠️ Como executar os exercícios

Como cada exercício foi criado como um binário independente dentro da pasta `src/bin`, você não utilizará apenas o comando padrão. 

Para rodar um arquivo específico, abra o terminal na raiz do projeto e utilize o comando `cargo run --bin` indicando o nome do arquivo (sem a extensão `.rs`). Por exemplo:

> `cargo run --bin ex01_variaveis`

---

## 📚 Trilha de Aprendizado (Fundamentos)

Abaixo está a lista completa dos exercícios abordados nesta etapa:

| Exercício | Arquivo | Conceito Principal |
| :--- | :--- | :--- |
| **01** | `ex01_variaveis.rs` | Tipos primitivos, variáveis imutáveis e `mut`. |
| **02** | `ex02_condicionais.rs` | Estruturas de controle (`if/else` e `match`). |
| **03** | `ex03_loopsv1.rs` / `v2.rs` | Laços de repetição e *Pattern Matching*. |
| **04** | `ex04_funcoes.rs` | Declaração de funções e recursividade. |
| **05** | `ex05_vetores.rs` | Criação de coleções dinâmicas (`Vec`) e alocação no *Heap*. |
| **06** | `ex06_vetores_func.rs` | O conceito de **Ownership** na prática (erro didático). |
| **07** | `ex07_borrowing.rs` | Passagem por referência imutável (`&`). |
| **08** | `ex08_borrowing_mut.rs` | Empréstimo mutável (`&mut`) e alteração de memória. |
| **09** | `ex09_structs.rs` | Criação de estruturas customizadas e trait `Debug`. |
| **10** | `ex10_impl.rs` | Implementação de métodos associados à Struct. |

---

## 💡 Dica

Se o compilador do Rust exibir avisos em amarelo (como o *dead_code* ou alertas de *snake_case*), não se preocupe! O seu código compilará perfeitamente. Esses avisos são apenas o Rust te ajudando a manter as melhores práticas de escrita de código.