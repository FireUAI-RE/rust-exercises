# Rust - Conceitos Básicos

Até hoje, a maioria dos exercícios que vocês fizeram possuía código parcialmente pronto, funções a completar ou uma estrutura já definida. Nesta atividade, você receberá apenas a especificação de cada programa e vai precisar decidir como organizar o código. 

# Instruções Gerais

- crie um projeto novo com `cargo new`
- organize o programa em funções sempre que fizer sentido
- não utilize crates externas
- o programa deve compilar sem warnings do `cargo clippy`(os amarelos)
- execute `cargo fmt` antes da entrega
- entradas inválidas fornecidas pelo usuário não devem causar `panic`
- não é necessário utilizar iterators, closures, macros ou outras features mais avançadas, mesmo que você já as conheça.
- A organização do programa faz parte do exercício.
- Cada atividade indica os principais conceitos para que você possa cuidar da sua revisão.

---

# Exercício 1 — Estatísticas

**Conceitos:** funções e `Vec`.

Escreva um programa que receba uma sequência de números **inteiros** pela entrada padrão e exiba algumas estatísticas sobre ela.

Exemplo:

```text
Entrada:
10 3 5 8 3 12

Saída:
Quantidade: 6
Soma: 41
Media: 6.83
Minimo: 3
Maximo: 12
```

O programa deve calcular:

- quantidade de valores;
- soma;
- média;
- menor valor;
- maior valor.

Separe os cálculos em funções. Pense em quais devem ser seus argumentos e quais valores deveriam retornar. O programa também deve lidar corretamente com uma entrada vazia.

# Exercício 2 — Contador de palavras

**Conceitos principais:** arquivos e `HashMap`.

Escreva um programa que receba o caminho de um arquivo de texto como argumento:

```bash
cargo run -- texto.txt
```

O programa deve imprimir:

```text
Linhas: 42
Palavras: 318
Palavra mais frequente: Nix (17)
```

Considere palavras como qualquer `string` separadas por espaços em branco. Para a contagem da palavra mais frequente, `"Rust"` e `"rust"` devem ser consideradas a mesma palavra, ou seja, case-insensitive.

Caso o arquivo não exista ou não possa ser lido, mostre uma mensagem de erro adequada em vez de causar `panic`.

# Exercício 3 — Agenda

**Conceitos principais:** `struct` e métodos.

Crie uma pequena agenda de contatos executada interativamente pelo terminal.

O programa deve aceitar os seguintes comandos:

```text
ADD Nathan 31999999999
ADD Lior 31888888888
GET Lior
REMOVE Nathan 
LIST
QUIT
```

Uma possível saída seria:

```text
Nathan adicionado
Lior adicionado
Lior: 31999999999
Nathan removido
Lior: 31999999999
```

Cada contato deve ser representado por uma `struct`.

A agenda também deve possuir uma estrutura própria responsável por armazenar os contatos e realizar operações sobre eles.

A implementação deve suportar:

- adicionar contato;
- consultar contato pelo nome;
- remover contato;
- listar todos os contatos;
- encerrar o programa.

# Exercício 4 — interpretador de comandos

**Conceitos principais:** `enum` e tratamento de erros.

Crie um programa que mantenha um conjunto de variáveis inteiras em memória e interprete comandos fornecidos pelo usuário.

Exemplo:

```text
SET gap 8000
SET coffee 30
ADD gap 1
SUB coffee 10
PRINT gap
PRINT
QUIT
```

Saída esperada:

```text
gap = 8001
coffee = 20
```

Os comandos disponíveis são:

```text
SET <nome> <valor>
ADD <nome> <valor>
SUB <nome> <valor>
PRINT <nome>
PRINT
QUIT
```

Use um `enum` para representar internamente os diferentes tipos de comando.

O programa deve separar, de alguma forma, duas responsabilidades:

1. transformar uma linha de texto em um comando;
2. executar esse comando sobre o estado atual do programa.

Entradas como:

```text
COMANDO
ADD hp mana
SET
```

devem produzir uma mensagem de erro e permitir que o programa continue rodando.

Não utilize `unwrap()` para processar dados fornecidos pelo usuário.

# Projeto — Task Tracker

**Conceitos principais:** composição de abstrações e persistência em arquivo.

Agora construíremos um programa um pouco maior reutilizando as ideias dos exercícios anteriores.

O programa deve funcionar como um pequeno gerenciador de tarefas pela linha de comando.

Exemplos:

```bash
cargo run -- add "Comprar um andador pra Natália"
cargo run -- add "Trancar Probabilidade"
cargo run -- list
```

Saída:

```text
[0] [ ] Comprar um andador pra Natália 
[1] [ ] Trancar Probabilidade
```

Também devem existir:

```bash
cargo run -- done 0
cargo run -- remove 1
cargo run -- show 0
```

Depois de:

```bash
cargo run -- done 0
```

a listagem deve mostrar:

```text
[0] [x] Comprar um andador pra Natália 
```

## Requisitos

Cada tarefa deve possuir pelo menos:

- identificador;
- descrição;
- estado indicando se foi concluída.

As tarefas devem ser representadas por uma `struct`.

O programa deve possuir alguma estrutura responsável pela coleção de tarefas e pelas operações realizadas sobre ela.

Os dados devem continuar existindo entre diferentes execuções do programa. Para isso, salve as tarefas em um arquivo local.

Você pode escolher um formato simples, por exemplo:

```text
0|true|Terminar atividade de Rust
1|false|Estudar assembly
```

Não utilize `serde` ou outro crate de serialização nesta atividade.

O programa deve implementar:

```text
add <descricao>
list
show <id>
done <id>
remove <id>
```

Casos como um ID inexistente, arquivo ainda não criado ou comando inválido devem ser tratados sem `panic`.

## Organização
Evite concentrar toda a implementação dentro de `main`.

Ao terminar, observe seu próprio código e tente identificar claramente:

- onde os argumentos da I/O são interpretados
- onde as tarefas são armazenadas e modificadas
- onde o arquivo é carregado e salvo
- onde a saída para o usuário é produzida

# Entrega
Crie uma branch do Git e um PR contendo os cinco exercícios nesse formato:

```text
atividade-rust/
├── estatisticas/
├── contador-palavras/
├── agenda/
├── interpretador/
└── task-tracker/
```

Dentro do seu diretório dessa semana.

Cada projeto deve poder ser executado normalmente utilizando `cargo run`.
Foque em:

- funcionamento correto
- legibilidade
- separação de responsabilidades
- uso adequado das estruturas básicas da linguagem
- capacidade de tratar entradas inválidas
- clareza da modelagem utilizada

Não invente moda tentando buscar a solução mais curta ou mais sofisticada. Prefira código simples que você consiga explicar.
