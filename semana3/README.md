# Semana 3 - Structs e Enums
Já usamos `structs` e `enums` na primeira atividade, mas agora é hora de tratar 

Já usamos `structs` e `enums` na primeira atividade, mas agora é hora de tratar dessas duas estruturas separadamente. Em geral, o objetivo dessa semana é que vocês aprendam como modelar dados com `structs` e `enums`, entendam como o `match` obriga a tratar todos os casos e como `Option` substitui o `null` ([e como ele resolve um problema de um milhão de dólares](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/)).

## Instruções Gerais

- crie um projeto novo com `cargo new`
- organize o programa em funções sempre que fizer sentido
- não utilize crates externas
- o programa deve compilar sem warnings do `cargo clippy` (os amarelos)
- execute `cargo fmt` antes da entrega
- entradas inválidas fornecidas pelo usuário não devem causar `panic`
- não defina traits nem tipos genéricos próprios, ainda não chegamos lá
- não é necessário utilizar iterators, closures, macros ou outras features mais avançadas, fica a seu critério.
- A organização do programa faz parte do exercício.
- Cada atividade indica os principais conceitos para que você possa revisar quando quiser

## Introdução

Tipos como `u32` e `String` não são a melhor forma de definir bem os limites de um dado mais complexo. Por exemplo: um usuário tem nome, idade e estado, e por isso é um forte candidato ã `struct`. Porém, um comando pode ser "adicionar", "remover" ou "sair", o que torna ele um perfeito candidato à `enum`. Uma abstração útil é que struct representa AND (nome E idade E estado) e enums representam OR (adicionar OU remover OU sair). Algumas diferenças gerais de outras linguagens:

- a `struct` é parecida com a do C, mas também pode ter **métodos**, como uma `class` do C++.
- o `enum` do C é só um nome para um inteiro e, por isso, ele é extremamente limitado. O enum do Rust pode **carregar dados** diferentes em cada variante. A forma mais próxima de descrever essa estrutura para um programador C é que eles funcionam como uma *tagged union* segura: o compilador cuida da tag para você e não deixa ler o dado errado. Pesquisem sobre esse conceito, ele é muito famoso no Zig!

## Structs

Uma `struct` agrupa valores de **tipos diferentes**, cada um com seu nome:

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("Nathan"),
        age: 20,
        active: true,
    };

    println!("{}", user.name);
}
```

Por conta do sistema de tipos rígido, a mutabilidade é da **instância inteira da struct**, não de cada campo, ou seja, não dá para marcar só `age` como `mut`:

```rust
let mut user = User {
    name: String::from("Felps"),
    age: 20,
    active: true,
};

user.age += 1;
```

Quando uma variável tem o mesmo nome do campo, existe um açucar sintático maravilhoso que nos permite abreviar:

```rust
fn new_user(name: String, age: u32) -> User {
    User {
        name,
        age,
        active: true,
    }
}
```

Também é possível usar uma `struct` como "template" para criar outra com `..`:

```rust
let other = User {
    name: String::from("Natália"),
    ..user
};
```

Dito isso, essa operação merece certo cuidado:

Os campos que vêm do `..user` são copiados ou **movidos**, conforme o tipo e as regras de `ownership` da semana passada.

Veja que aqui só `age` e `active` vêm de `user`, e os dois são tipos simples, logo, implementam copy `Copy`. Por isso, o primeiro `user` continua válido. Por outro lado, se o `name` também viesse de lá, `user` deixaria seria dropado e por consequência inutilizáel. Voltamos ao fantasma do Borrow Checker.

Existem ainda as *tuple structs*, com campos sem nome:

```rust
struct Color(u8, u8, u8);

let red = Color(255, 0, 0);
println!("{}", red.0);
```

Ótimos quando o dado a ser representado é simples, como o caso de `Color`

### Métodos

Métodos são funções definidas dentro de um bloco `impl`. O primeiro parâmetro diz **como** o método usa a struct. Dá pra mapear exatamente com as três formas que vocês viram na semana passada:

`self` -> consome a struct -> ownership
`&self`-> só lê os campos -> referência imutável
`&mut self` ->  modifica os campos -> referência mutável

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn grow(&mut self, amount: u32) {
        self.width += amount;
        self.height += amount;
    }
}

fn main() {
    let mut rect = Rectangle::new(3, 4);

    rect.grow(1);

    println!("{}", rect.area()); // 20
}
```

O `new` não tem `self`, então é uma *função associada*, chamada com `::`. Esse nome, em geral, é o jeito comum de escrever um "construtor" (como em `String::new()`. Já `grow` e `area` são métodos, chamados com `.`.

Repare que você escreve `rect.grow(1)` e não `(&mut rect).grow(1)` porque o Rust é a melhor linguagem do mundo e coloca a referência para você. Ou seja, não existe um `->` como no C.

Como último detalhe de implementação, veja que a chamada de `.grow()` nada mais é que um açucar sintático:

``` rust
fn grow(&mut self, amount: u32) {
	self.width += amount;
	self.height += amount;
}

rectangle.grow(amount);

```

Debaixo dos panos, quando você usa um `impl`, o compilador cria uma função separada e chama ela, executando um código mais próximo de:

```rust
fn grow(&mut Rectangle, amount: u32) {
	self.width += amount;
    self.height += amount;
}

grow(rectangle, amount);
```

Nós só seguimos o segundo método por conveniência, mas toda essa ideia de criar métodos que vem de POO nada mais é que perfumaria em cima de programação procedural básica.

### `derive` e `Debug`

Por padrão, uma `struct` não sabe se imprimir. 

Por sorte, outra das grandes forças do Rust é o uso de metaprogramação, basta você pedir por favorzinho que o compilador implementa isso para você:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 4, height: 5 };

    println!("{:?}", rect);  // Rectangle { width: 4, height: 5 }
    println!("{:#?}", rect); // o mesmo, em várias linhas
}
```

Sem o `#[derive(Debug)]`(que é um macro), o `{:?}` não compila. Mais específicamente,`Debug` é um *trait*. Todos os dois tópicos são assuntos que vamos ver em breve.

## Enums

Um `enum` é um tipo cujo valor é **uma entre várias variantes** (lembra do OR?):

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let d = Direction::North;
```

As variantes ficam dentro do nome do tipo e são acessadas com `::`. Diferente do C, `Direction::North` não é um inteiro: não dá para somar nem comparar com `0`, ele é um tipo completo.

O que torna o `enum` de Rust especial é que cada variante pode **guardar dados**, e cada uma pode guardar dados diferentes:

```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Point,
}
```

Não tenho palavras para explicar o tanto que isso é poderoso, você consegue modelar quase qualquer programa mais mundano como uma máquina de estados.

### `match`

Para descobrir qual variante você tem e acessar seus dados, usa-se `match`, mais especificamente, ele implementa pattern matching na linguagem:

```rust
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Point => 0.0,
    }
}
```

Em detalhes: `match` é uma expressão onde cada braço produz um valor, e o conjunto todo produz o valor da função. Como `shape` é uma referência, `radius`, `width` e `height` também são referências para os dados dentro do `enum`, mas o Rust deixa fazer conta com elas direto.

Porém, a coisa ainda melhora, não bastasse o tanto que a modelagem é poderosa, o compilador ainda **garante** que você cobra todas as variantes. Quer testar? Remova a linha do `Point` e leia o erro. 

Isso torna extremamente mais seguro evoluir o código: ao adicionar uma variante nova, o compilador mostra todos os `match` que precisam ser atualizados, mesma coisa para qualquer função que receba esse `Enum`.

O `match` também aceita valores, intervalos, várias opções com `|`, condições com `if` e um braço curinga `_` para "todo o resto":

```rust
fn describe(n: i32) {
    match n {
        0 => println!("zero"),
        1 | 2 => println!("pouco"),
        3..=9 => println!("alguns"),
        n if n < 0 => println!("negativo"),
        _ => println!("muitos"),
    }
}
```

Use o `_` com cuidado. Ele silencia justamente o aviso que te protege quando uma variante nova aparece.

A combinação desse funcionamento é uma das maiores belezas de programar em Rust, você consegue encapsular grande parte do estado do seu programa em `enums` e garantir que as transições funcionam exatamente de acordo com o conjunto de regras que você espera. Isso torna a gestão do estado mais explícita e facilita muito tanto o processo de entendimento como de debug do código.

### `Option`

Agora que sabemos como `Enums` funcionam, podemos partir para um `Enum` especial: o `Option`

Rust não tem `null` e isso é uma escolha proposital. Um valor que pode não existir é representado por esse `enum` da biblioteca padrão:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

O `<T>` quer dizer "de qualquer tipo", como em `Vec<T>`. Criar seus próprios tipos genéricos vai ser outro tópico que veremos em breve.

Vocês já encontraram `Option` em coisas como `Vec::pop()`.

Como `Option<i32>` é um tipo diferente de `i32`, o compilador não deixa você usar um valor que pode estar ausente como se ele existisse. Para chegar ao valor é preciso tratar o `None`, ou seja, "desempacotar" o `i32`, garantindo que ele não está vazio:

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for n in numbers {
        if n % 2 == 0 {
            return Some(*n);
        }
    }

    None
}

fn main() {
    let numbers = vec![1, 3, 4, 5];

    match find_first_even(&numbers) {
        Some(n) => println!("achei {n}"),
        None => println!("nenhum par"),
    }
}
```

Fazer `match` é bom para ser explícito, mas as vezes nós só ligamos para o valor se ele existir, para isso temos um outro açúcar sintático, o `if let`:

```rust
if let Some(n) = find_first_even(&numbers) {
    println!("{n}");
}
```

Também temos a função `unwrap_or`, que dá um valor padrão para o `None`:

```rust
let n = find_first_even(&numbers).unwrap_or(0);
```

Já o `unwrap()` simplesmente dá `panic` se o valor for `None`. É por isso que ele está proibido para dados vindos do usuário, imagina crashar o programa toda vez que o usuário for burro?

### Juntando tudo

`struct`, `enum` e `impl` combinam bem. Um comando de um programa, por exemplo, é naturalmente um `enum`:

```rust
enum Command {
    Add(String),
    Remove(u32),
    List,
    Quit,
}

impl Command {
    fn describe(&self) -> String {
        match self {
            Command::Add(text) => format!("adicionar {text}"),
            Command::Remove(id) => format!("remover a tarefa {id}"),
            Command::List => String::from("listar tarefas"),
            Command::Quit => String::from("sair"),
        }
    }
}
```

## Exercícios de Fixação

Esses são para treinar sua mente e te colocar para pensar, não precisa entregar. Façam os qu esentirem que vão te ajudar.

### 1\. Compila? Explique

a)

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };

    p.x = 10;

    println!("{} {}", p.x, p.y);
}
```

b)

```rust
enum Light {
    Red,
    Yellow,
    Green,
}

fn seconds(light: Light) -> u32 {
    match light {
        Light::Red => 30,
        Light::Green => 25,
    }
}
```

c)

```rust
struct Counter {
    value: u32,
}

impl Counter {
    fn finish(self) -> u32 {
        self.value
    }
}

fn main() {
    let c = Counter { value: 3 };

    let total = c.finish();

    println!("{} {}", total, c.value);
}
```

Se não compilar, corrija o código mudando o **mínimo possível**.

### 2\. Implemente

Na `struct Rectangle` de antes, escreva um método `can_hold` que diga se um retângulo cabe inteiro dentro de outro:

```rust
fn can_hold(&self, other: &Rectangle) -> bool
```

### 3\. Adicione uma variante

No `enum Shape` e na função `area` de antes, escreva também uma função `perimeter`. Depois adicione a variante `Triangle { a: f64, b: f64, c: f64 }` e observe quais `match` deixam de compilar.

### 4\. Implemente com `Option`

Escreva `fn average(numbers: &[f64]) -> Option<f64>`, que devolve `None` para uma lista vazia, em vez de dividir por zero.

## Atividade 1 — Leitura
Caso queira se aprofundar mais leia os capítulos 5 e 6 do livro The Rust Programming Language:

- Capítulo 5, structs: https://doc.rust-lang.org/book/ch05-00-structs.html
- Capítulo 6, enums e pattern matching: https://doc.rust-lang.org/book/ch06-00-enums.html

São capítulos curtos. Digite os exemplos, rode e quebre eles de propósito.

## Projeto 1 — Banco

Crie um pequeno banco, executado interativamente pelo terminal, que mantém várias contas. O programa deve aceitar os comandos:

```text
OPEN Nathan
OPEN Lior
DEPOSIT Lior 100
WITHDRAW Lior 30
BALANCE Lior
STATEMENT Lior
QUIT
```

Uma possível saída seria:

```text
Conta de Nathan aberta
Conta de Natália aberta
Depósito de 100.00 na conta de Natália
Saque de 30.00 na conta de Natália
Saldo de Natália: 70.00

Extrato de Natália:
+100.00
-30.00
Saldo: 70.00
```

O formato exato da saída é livre, mas ele deve mostrar claramente o resultado de cada comando.

Para valores, use `f64` e ignore por enquanto problemas de arredondamento.

### Requisitos

- Cada conta deve ser uma `struct` com pelo menos o nome do dono, o saldo e um histórico de movimentações.
- Cada movimentação do histórico deve ser uma `struct` própria, com ao menos uma descrição e um valor (positivo para depósito, negativo para saque).
- Deve existir uma `struct` para o banco, que guarda as contas e realiza as operações sobre elas.
- Operações de uma conta devem ser **métodos** dela. Criar uma conta deve ser uma função associada. Pense em quais métodos precisam de `&self` e quais precisam de `&mut self`.
- Procurar uma conta pelo nome deve devolver um `Option`.
- Os seguintes casos devem ser tratados com uma mensagem, sem `panic`, e o programa deve continuar rodando: conta inexistente, conta já existente, valor menor ou igual a zero, saldo insuficiente e comando inválido.
- Não utilize `clone()` apenas para contornar problemas de borrowing.

**Desafio extra(opcional):** adicione `TRANSFER Ana Bruno 50`. Você vai descobrir que o borrow checker não deixa pegar duas referências mutáveis para elementos do mesmo `Vec` ao mesmo tempo. Pense em como resolver isso sem `clone()`.

## Projeto 2 — Pedidos

O objetivo desse exercício é demonstrar o que eu quis dizer com modelar uma máquina de estados.

Crie um programa que gerencia pedidos de um restaurante. Cada pedido passa por estados bem definidos, e é isso que o `enum` modela muito bem:

```rust
enum OrderStatus {
    Pending,
    Preparing { chef: String },
    Delivered { minutes: u32 },
    Cancelled(String),
}

struct Order {
    id: u32,
    items: Vec<String>,
    status: OrderStatus,
}
```

As transições válidas são:

- um pedido **pendente** passa a **em preparo**, informando o nome do cozinheiro;
- um pedido **em preparo** passa a **entregue**, informando quantos minutos levou no total;
- um pedido **pendente** ou **em preparo** pode ser **cancelado**, informando o motivo;
- pedidos **entregues** ou **cancelados** não mudam mais.

Qualquer outra transição deve ser recusada com uma mensagem, sem alterar o pedido.

O programa deve permitir:

- criar um pedido com seus itens;
- avançar ou cancelar um pedido existente;
- listar todos os pedidos com o seu estado;
- mostrar um resumo com a quantidade de pedidos em cada estado e o tempo médio de entrega.

Exemplo do que a listagem e o resumo poderiam mostrar:

```text
[1] pizza, coca: entregue em 35 min
[2] salada: cancelado (sem estoque)
[3] hamburguer: em preparo por Marcos
[4] sushi: pendente

Pendentes: 1
Em preparo: 1
Entregues: 1
Cancelados: 1
Tempo médio de entrega: 35.0 min
```

**Você decide como será a interface**, mas em compensação ela **deve ser documentada**. Basta explicar em um `README.md`, junto da entrega, como fazer cada operação. Não precisa de muitos detalhes, só uma explicação simples para eu entender.

### Requisitos

- `OrderStatus` deve ser um `enum` com pelo menos as quatro variantes acima, cada uma guardando os dados indicados.
- Cada transição deve ser um **método** de `Order` que usa `match` para conferir o estado atual. Pense em qual `self` cada um precisa.
- Em pelo menos um `match`, como o que gera o texto do estado, **não use o braço `_`**. Liste todas as variantes, para que o compilador avise se uma nova for adicionada.
- O tempo médio de entrega deve ser calculado por uma função que devolve `Option<f64>`, com `None` quando nenhum pedido foi entregue.
- Procurar um pedido por id deve devolver um `Option`, e pedido inexistente deve ser tratado sem `panic`.
- Não utilize `unwrap()` para processar dados fornecidos pelo usuário.
- Procure usar as referências seguindo o conceito do **menor privilégio**.
- Não utilize `clone()` apenas para contornar problemas de borrowing.

## Entrega

```text
semana3/
├── banco/
└── pedidos/
```

Dentro do seu diretório dessa semana.

Cada projeto deve poder ser executado normalmente utilizando `cargo run`.

Foque em:

- funcionamento correto
- legibilidade
- separação de responsabilidades
- uso adequado de `struct`, `enum` e `match`
- uso de referências seguindo o conceito do menor privilégio
- capacidade de tratar entradas inválidas
- clareza da modelagem utilizada

Antes de escrever código, pode ajudar desenhar os dados no papel: o que é `struct`, o que é `enum`, e o que cada método precisa receber (`self`, `&self` ou `&mut self`). Quase todo o trabalho dessa semana está nessa decisão.

Não invente moda tentando buscar a solução mais curta ou mais sofisticada. Prefira código simples que você consiga explicar.
