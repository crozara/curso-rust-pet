// Exercicio 07 - Borrowing
//-----------------------------------------------------------------
//Corrija o exercício anterior usando as técnicas de borrowing e 
//seus conhecimentos sobre o ownership.

//Solução: Agora recebemos UMA REFERÊNCIA do vetor (&Vec<String>)
//A função apenas "pega emprestado", sem roubar o Ownership.


fn contar(vec: &Vec<String>) -> i16 {
    vec.len() as i16
}

fn main (){
    
    let mut vetor: Vec<String> = vec![
        String::from("Kaue"), 
        String::from("Crozara"), 
        String::from("Artur"), 
        String::from("Silva"), 
        String::from("JP")
    ];

    println!("{:?}", vetor);

    vetor.push(String::from("Meu nome"));
    println!("{:?}", vetor);

    //Passamos o vetor por referência usando o '&'
    println!("Tamanho do vetor: {}", contar(&vetor));

    //Agora funciona, como o vetor foi apenas emprestado para a função a main() nunca perdeu a posse dele.
    println!("{:?}", vetor);

}