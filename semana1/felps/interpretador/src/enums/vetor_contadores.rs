use super::contador::Contador;

pub fn procura_contador<'a>(
    contadores: &'a mut Vec<Contador>,
    nome: &str,
) -> Option<&'a mut Contador> {
    // uso de lifetime
    // entender melhor depois
    for contador in contadores {
        if nome == contador.nome {
            return Some(contador);
        }
    }

    None
}

pub fn print_all(contadores: &Vec<Contador>) {
    for contador in contadores {
        contador.print();
    }
}
