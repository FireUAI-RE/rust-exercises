
pub fn calcula_quantidade(numeros: &Vec<i32>) -> usize{
    numeros.len()
}

pub fn calcula_soma(numeros: &Vec<i32>) -> i32{
    let mut soma = 0;
    for i in 0..=numeros.len()-1{
        soma +=numeros[i];
    }
    soma
}

pub fn calcula_max_min(numeros: &Vec<i32>) -> (i32, i32){
    let mut max = numeros[0];
    let mut min = numeros[0];
    for i in 1..=numeros.len()-1{
        if numeros[i] < min{
            min = numeros[i];
        } 
        if numeros[i] > max{
            max = numeros[i];
        }
    }
    (max, min)
}


