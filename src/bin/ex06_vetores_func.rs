// Exercicio 06 - Vetores com Funções
//-----------------------------------------------------------------
//Crie uma função que receba o vetor de nomes e retorne a quantidade de elementos que ele possui
//(dica: utilize o método .len()). Na função main, chame essa função para imprimir a quantidade de nomes e,
//logo em seguida, tente imprimir o próprio vetor completo.

fn contar(vec: Vec<String>) -> i16 {
    
    //Utilizando o 'as' para relembrar o Casting de tipos,
    //já que o .len() retorna um usize nativamente.
    vec.len() as i16
}

fn main (){
    //Mantendo o padrão de alocação do Exercicio 05
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

    //Aqui o OwnerShip do vetor é transferido para a função contar()
    println!("Tamanho do vetor: {}", contar(vetor));

    //DESCOMENTE PARA VISUALIZAR O ERRO (Borrow of moved value): 
    // println!("{:?}", vetor);

}