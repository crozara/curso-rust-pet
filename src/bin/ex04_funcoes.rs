//Exercicio 04 - Funções
//-----------------------------------------------------------------
//Faça uma função para calcular potência de algum número.
//Se souber recursividade, dê preferência para fazer uma função recursiva

//Fazendo a criação da função de maneira recursiva
fn potencia(base: i32, exp: i32) -> i32 {
    if exp == 1 {
        return base;
    }
    else if exp == 0 {
        return 1;
    }

    // Retorno implícito (sem a palavra 'return' e sem o ';')
    base * potencia(base, exp-1)
}

fn main() {
    //Fazendo a chamada da função direto no print.
    println!("Potencia: {}", potencia(2, 10))
}
