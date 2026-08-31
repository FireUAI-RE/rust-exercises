pub fn calcula_quantidade(numeros: &[i32]) -> usize {
    numeros.len()
}

pub fn calcula_soma(numeros: &[i32]) -> i32 {
    let mut soma = 0;
    let mut i = 0;
    
    while i < numeros.len() {
        soma += numeros[i];
        i += 1;
    }
    
    soma
}

pub fn calcula_max_min(numeros: &[i32]) -> (i32, i32) {
    let mut max = numeros[0];
    let mut min = numeros[0];
    let mut i = 0;
    while i < numeros.len(){
        if numeros[i] < min {
            min = numeros[i];
        }
        if numeros[i] > max {
            max = numeros[i];
        }
        i += 1;
    }
    (max, min)
}
