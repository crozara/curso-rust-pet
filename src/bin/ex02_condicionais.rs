//Exercicio 02 - Condicionais.
//-----------------------------------------------------------------
//Use match para classificar números entre intervalos:
//Números negativos → Imprima que é um numero negativo
//Números entre 0 e 1000 (com os dois inclusos) → Imprima uma mensagem informando que esta dentro deste intervalo
//Números maiores que 1000 → Imprima uma mensagem informando que esta dentro deste intervalo

fn main()
{
    //Fazemos a criação da variavel
    let num: i32 = 10000;

    //Utilizamos o Pattern matching para fazer todas as verificações
    match num {
        ..0 => println!("O número é negativo."),
        0..=1000 => println!("Está entre 0 e 1000."), 
        //Lembrando que o '_' significado o caso default.
        _ => println!("O número é maior que 1000.") 
    }
}
