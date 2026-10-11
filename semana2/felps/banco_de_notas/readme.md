# Projeto 2 - Banco de Notas

Um programa em Rust desenvolvido via CLI para gerenciar o cadastro de estudantes e registrar suas notas. 

## Como executar

Para iniciar o programa interativo, rode na raiz do projeto:
`bash
cargo run
`

## Comandos Disponíveis (Interface)

A interface aguarda comandos digitados pelo usuário. Os comandos diferenciam letras maiúsculas de minúsculas.

* **`ADD <nome>`** 
  Adiciona um novo estudante ao banco de dados. 
  *Ex: `ADD Thiago`*

* **`GRADE <nome> <titulo_atividade> <nota> <nota_maxima>`**
  Registra uma nova nota para o estudante especificado.
  *Ex: `GRADE Thiago Prova1 2.5 10.0`*

* **`AVERAGE <nome>`**
  Calcula e exibe a média das notas do estudante pesquisado.
  *Ex: `AVERAGE Thiago`*

* **`SUM <nome>`**
  Exibe a soma total de pontos obtidos pelo estudante em relação ao total de pontos.
  *Ex: `SUM Thiago`*

* **`LIST`**
  Lista todos os estudantes atualmente registrados no sistema.
  
* **`INFO`**
  Exibe todos os comandos.

* **`QUIT`**
  Encerra o programa.

## Exemplo de Uso

`text
> ADD Natália
> ADD Nathan
> GRADE Nathan Prova1 8.5 10.0
> GRADE Natália Trabalho 7.0 10.0
> AVERAGE Nathan
Nathan
Average: 8.5
> LIST
STUDENTS:
Natália
Nathan
> QUIT
`