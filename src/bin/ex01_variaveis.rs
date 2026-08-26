//Exercicio 01 - Variaveis.
//-----------------------------------------------------------------
//Guarde o nome de 3 pessoas e suas respectivas idades. Após isso,
//mostre a média das idades e, por fim, mude o nome e idade da
//primeira pessoa, recalcule a média e mostre-a novamente.

fn main() {

    //Fazendo a criação das variaveis pessoas:
    let mut _pessoa1: String = String::from("Kauê");
    let _pessoa2: String = String::from("João Pedro");
    let _pessoa3: String = String::from("Rust");

    //Fazendo a criação das variavies idade;
    let mut idade1: i32 = 18;
    let idade2: i32 = 20;
    let idade3: i32 = 30;

    //Fazendo a criação da variavél que vai guardar a média (Lembrar de converter para float)
    let media: f64 =  (idade1 + idade2 + idade3) as f64 / 3.0;

    //Mostrando a média das idades:
    println!("A média das idades é: {media}");

    //Mudando o nome e idade da primeira pessoa (lembrar de usar o 'mut')
    _pessoa1 = String::from("Crozara");
    idade1 = 20;

    let media: f64 =  (idade1 + idade2 + idade3) as f64 / 3.0;
    println!("A nova média das idades é: {media}");


}
