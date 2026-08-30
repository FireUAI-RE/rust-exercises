use super::contador::Contador;

pub fn procura_contador<'a>(
    contadores: &'a mut [Contador],
    nome: &str,
) -> Option<&'a mut Contador> {
    // uso de lifetime
    // entender melhor depois
    let mut i = 0;
    while i < contadores.len() {
        if nome == contadores[i].nome {
            return Some(&mut contadores[i]);
        }
        i += 1;
    }
    None
}

pub fn print_all(contadores: &Vec<Contador>) {
    for contador in contadores {
        contador.print();
    }
}
