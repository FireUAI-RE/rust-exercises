# Rust — Borrowing
 
Agora que a maioria dos conceitos básicos foi estabelecida na primeira semana, vamos começara entrar em tópicos, features e especificidades da linguagem em si, começando pelo que permite que rust seja seguro: **O Borrow Checker**.

# Instruções Gerais 

- crie um projeto novo com `cargo new`
- organize o programa em funções sempre que fizer sentido
- não utilize crates externas
- o programa deve compilar sem warnings do `cargo clippy`(os amarelos)
- execute `cargo fmt` antes da entrega
- entradas inválidas fornecidas pelo usuário não devem causar `panic`
- não é necessário utilizar iterators, closures, macros ou outras features mais avançadas, fica à seu critério.
- A organização do programa faz parte do exercício.
- Cada atividade indica os principais conceitos para que você possa revisar quando quiser 

# Introdução
Rust é uma linguagem linda, mas de vez em quando ela entra no nosso caminho mesmo ao escrever os programas mais simples que existem, considere por exemplo o seguinte:

``` rust
fn print_name(name: String) {
    println!("{name}");
}

fn main() {
    let name = String::from("Thiago");

    print_name(name);

    println!("{name}");
}
```

Esse programa simplesmente não compila. Isso é frustrante, e às vezes dá vontade de socar o teclado, mas existe um ótimo motivo para isso, por sorte, o `rustc` é gentil e nos mostra como resolver esse erro:

``` text
error[E0382]: borrow of moved value: `name`
  --> teste.rs:10:16
   |
 6 |     let name = String::from("Thiago");
   |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
 7 |
 8 |     print_name(name);
   |                ---- value moved here
 9 |
10 |     println!("{name}");
   |                ^^^^ value borrowed here after move
   |
note: consider changing this parameter type in function `print_name` to borrow instead if owning the value isn't necessary
  --> teste.rs:1:21
   |
 1 | fn print_name(name: String) {
   |    ----------       ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
help: consider cloning the value if the performance cost is acceptable
   |
 8 |     print_name(name.clone());
   |                    ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

Se você for capaz de entender completamente tudo o que aparece nessa mensagem de erro, provavelmente você já sabe tudo o que abordaremos sobre o assunto. 

# Referências
Assim como no C, uma referência é um meio de apontar para um valor na memória e acessar seu conteúdo, conveninentemente, rust também utiliza o mesmo símbolo, `&` para definir uma referência.

``` rust
let name = String::from("Nathan");

let reference = &name;
```

Nesse caso, a variável `reference` tem tipo `&str`, ou seja, uma referência para uma `String`, funções podem deliberadamente pedir que seus parâmetros sejam referências. Veja que, utilizanod isso, o seguinte código compila sem nenhum problema:

``` rust
fn print_name(name: &str) {
    println!("{name}");
}

fn main() {
    let name = String::from("Thiago");

    print_name(&name);

    println!("{name}");
}
```

Por que? Chegaremos lá em breve.

Quando uma referência é criada, dizemos que estamos **emprestando** o valor, o que em inglês fica **borrowing**. Com base nessa feature, foi criada a maior vantagem da linguagem e, ao mesmo tempo, o seu maior boss: o **borrow checker**, que é basicamente o mecanismo interno do rust para garantir que você está utilizando essa feature corretamente sem vazar memória. Vamos tratar melhor dele mais a frente.

Assim como em C, *borrowing* não cria uma cópia do objeto, ao contrário, você pode pensar que é apenas um ponteiro para seu endereço de memória.

## Mutabilidade de Referências

Aqui começa a existir a principal diferença do modelo de C. Lá, você recebe com um ponteiro acesso ao endereço de memória que contém os dados e pode fazer o que quiser com ele. Em Rust, por padrão, as referências são **imutáveis**, ou seja, read-only. Por isso, essa função não compila:
``` rust
fn change(text: &str) {
    text.push('!');
}

```

A boa notícia é que podemos declarar referências mutáveis, porém elas devem ser explícitas, desse jeito:

``` rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("hello");

    add_exclamation(&mut text);

    println!("{text}");
}
```

Como além de segurança de memória Rust busca permitir paralelismo simplificado, para qualquer valor é possível smepre ter apenas:
- 1 Referência mutável
- OU (exclusivo) quantas referências imutáveis você quiser

Pense no porquê disso: Dessa forma, é impossível que haja uma *race condition*. Apenas um altera e ninguém lê ou todo mundo lê mas ninguém altera.

Além disso, o compilador é inteligente o bastante para facilitar sua vida e permitir que você não pense tanto sobre isso enquanto não encontra um caso crítico, veja, esse programa compila:

``` rust
let mut name = String::from("Nathan");

let a = &name;

println!("{a}");

let b = &mut name;

println!("{b}");
```

Nesse caso, o compilador sabe que depois de usar no primeiro `println!`, a referência `a` nunca é usada novamente, então ele deleta instantaneamente e permite a criação de `b` logo na sequência, porém, esse não compila:

``` rust
let mut name = String::from("Nathan");

let a = &name;

let b = &mut name;

println!("{a}");

println!("{b}");
```

Porque aqui não é possível deletar `a` prematuramente, percebe?

# Derreference
A operação contrária à de criar uma referência é desreferenciar (não diga!). Isso é justamente o mesmo que o C também, se você de fato entendeu o uso de * ao invés de considerar `int *` como um tipo:

``` rust
let mut x = 10;
let reference = &mut x;
*reference += 1;
```

# Deallocation Automático

Funciona da mesma forma que variáveis da *stack* no C, ao final do escopo, variáveis simples são limpas da memória:

``` rust
fn teste() {
	let x = 1;
	{
		let y = 2;
	} // y morre aqui
} // x morre aqui
```

# O Trait `Copy`
Não vou explicar em detalhes o que são traits, visto que não é o assunto dessa semana, porém esse comportamente é implementado par diferentes tipos, e é necessário que voc6e tenha uma intuição de como ele funciona.

Você concordaria que um `u32` não gasta muita memória correto? O compilador também concorda, por isso, quando você faz:

``` rust
let x = 5;
let y = x;
```

Ele simplesmente faz uma cópia do valor 5 para y, simples não? Isso ocorre pois o tipo `u32` implementa `Copy`.

Acontece que o mesmo não acontece para todos os tipos de uma linguagem. Por exemplo, uma `String` pode ter tamanho arbitrário (lembre-se que toda `String` é apenas um ponteiro para seu primeiro caractere) e, por isso, ela é custosa para ser copiada. Por isso, quando você faz isso:

``` rust
{
	let s1 = String::from("hello");
	let s2 = s1;
	println!("{s1}, world!");
}
```

O resultado se parece mais com isso:

[IMG1]

E não isso:

[IMG2]

Veja que, caso ao final do escopo ambas as referências para `String` se mantivessem, ambas tentariam liberar a memória e, por consequência, ocorreria um *double free*. Por isso, o compilador move o valor para `s2` e torna `s1` inválido, assim, recebemos esse erro:

``` text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |                ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

# O Sistema de Ownership 👻

Vamos matar esse monstro de 7 cabeças, ele segue algumas regras pré-determinadas:
- Cada valor no rust tem um  dono (*owner*) 
- Só pode haver um dono por vez 
- Quando seu dono sai de escopo, o valor correspondente é deletado da memória.

Acabou. Doeu?

Toda vez que você coloca um tipo concreto no argumento de uma função, você está movendo sua *ownership*, sempre que é uma referência você está fazendo *borrowing*. O mesmo vale para variáveis.

Apesar de simples, esse sistema é o bastante para impedir toda uma classe de falhas de memória e simplificar seu gerenciamento quando comparado ao C.

Eu poderia falar muito sobre isso e demonstrar as suas implicações, mas acredito que nesse caso é melhor sentir o sistema na prática.

# Exercícios de Fixação
Esses são para treinar sua mente e te colocar para pensar, não precisa entregar.

## 1. Compila? Explique

a)
``` rust
fn main() {
    let name = String::from("Rust");

    let reference = &name;

    println!("{name}");
    println!("{reference}");
}
```

b)
``` rust
fn main() {
    let mut x = 10;

    let a = &x;
    let b = &x;

    println!("{a}");
    println!("{b}");
}
```

c)

``` rust
fn main() {
    let mut x = 10;

    let a = &mut x;
    let b = &mut x;

    println!("{a}");
    println!("{b}");
}
```

## 2. Corrija o código sem `clone()`

``` rust
fn print_text(text: String) {
    println!("{text}");
}

fn main() {
    let text = String::from("hello");

    print_text(text);

    println!("{text}");
}
```

## 3. Implemente a função `fn increment(value: &mut i32)`, ela deve adicionar 1 ao número.

## 4. Corrija e explique o erro do código

a)

``` rust
fn add_value(values: &Vec<i32>) {
    values.push(10);
}
```

b)

``` rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    let first = &numbers[0];

    numbers.push(4);

    println!("{first}");
}
```

# Atividade 1
Leia o capítulo 4 do livro The Rust Programming Language:
https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

Ele é curto, sem medo.

# Projeto 1 - Analisador de Texto
Uma extensão simples do projeto anterior, mas agora vocês devem se preocupar especificamente com *borrowing* para pegar prática. Se acostume com os erros e saídas do compilador, você não deve demorar muito tempo lendo ele para entender o problema. 

Seu programa deve receber duas `String`s de entrada e produzir algumas informações, veja um exemplo:

``` text
Entrada:
Rust is fast and Rust is safe
Palavra procurada: Rust

Saída:
Caracteres: 28
Palavras: 7
Maior palavra: safe
Ocorrências de Rust: 2
```

Seu programa deve ser **case-insensitive**. Está claro no exemplo, mas mais especificamente as funcionalidades devem ser:
- contar caracteres;
- contar palavras;
- encontrar a maior palavra;
- contar quantas vezes uma determinada palavra aparece.

## Requisitos
- Nenhuma função que implementa uma das funcionalidades deve assumir *ownership* dos seus argumentos
- Procure usar as referências seguindo o **conceito do menor privilégio**

# Projeto 2 - Banco de notas

Crie um programa que mantenha uma coleção de estudantes e suas notas.

Estudantes devem ser modelados, ao menos minimamente, como:

``` rust
struct Student {
    name: String,
    grades: Vec<f32>,
}
```

O programa deve permitir cadastrar estudantes e adicionar notas.

**Você pode decidir como será a interface**, mas em compensação, ela **deve ser documentada** junto com o projeto. Não precisa ser extensa, basta explicar como fazer cada operação em um README.md junto da sua entrega.

Um exemplo de interface:

Entrada:
``` rust
ADD Natália
ADD Nathan

GRADE Nathan 8.5
GRADE Natália 7.0
GRADE Nathan 9.0

SHOW Nathan
AVERAGE Nathan
LIST
```

Saída:

``` rust
Nathan
Notas: 8.5, 7.0
Média: 7.75
```

## Requisitos
Além da estrutura para encapsular os estudantes, as funcionalidades mínimas devem ser:

- adicionar estudante;
- adicionar nota;
- procurar estudante;
- calcular média;
- listar estudantes.

Além disso:
- Você deve decidir razoavelmente que função deve ou não deve assumir ownership.
- Procure usar as referências seguindo o **conceito do menor privilégio**
- estudantes inexistentes devem ser tratados sem panic;
- notas inválidas devem ser rejeitadas;
- não utilize clone() apenas para contornar problemas de borrowing;
