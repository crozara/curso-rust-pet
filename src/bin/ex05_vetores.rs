// Exercicio 05 - Vetores
//-----------------------------------------------------------------
// Faça um VECTOR de String com os 5 nomes das pessoas mais próximas de você e imprima esse vector.
// Após isso, adicione o seu nome ao vector e o imprima novamente.

fn main() {
    // Criando o vetor de Strings
    let mut vetor: Vec<String> = vec![
        String::from("Kaue"), 
        String::from("Crozara"), 
        String::from("Artur"), 
        String::from("Silva"), 
        String::from("JP")
    ];

    // Lembrando que ":?" significa que vamos imprimir todo o vetor, sem ter a necessidade de fazer um laço.
    println!("{:?}", vetor);

    // Utilizamos o "push" para adicionar e convertemos o texto para String também
    vetor.push(String::from("Meu nome"));
    
    println!("{:?}", vetor); 
}