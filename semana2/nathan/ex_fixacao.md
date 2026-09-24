# Exercicios de Fixação

## 1

a) O código compila e executa normalmente devido ao valor não trocar a ownership com o &, portanto, pode-se dar o print as duas vezes, o dono não se altera.

b) O código compila e executa normalmente, mas nesse caso com warning, pois não havia a necessidadde de ser mutável, x só é referenciado.

c) Nesse caso não compila, pois o valor não está somente sendo referenciado ele é emprestado duas vezes seguidas para poder ser alterado e isso é proibido.

## 2

```
fn print_text(text: &String) { // Corrigi emprestando os valores
    println!("{text}");
}

fn main() {
    let text = String::from("hello");

    print_text(&text); // Corrigi emprestando os valores

    println!("{text}");
}
```

## 3
```
fn increment_value(num: &mut i32) {
    *num = *num + 1;
}

fn main() {
    let mut value: i32 = 0;
    increment_value(&mut value);
    println!("Numero +1: {value}");
}
```

# 4

a) Nesse caso deveria ser ```fn add_value(values: &mut Vec<i32>)``` pois antes só fazia referência, não dizia que podia alterá-lo

b) A correção fica ```let first = numbers[0];``` pois, assim, você cria uma cópia ao invés de emprestar e tentar usar depois no print.
