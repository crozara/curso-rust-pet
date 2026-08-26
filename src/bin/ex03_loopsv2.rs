//Exercicio 03 - Loops - Resolução utilizando Pattern Matching.
//-----------------------------------------------------------------
//Percorra números de 1 a 50 (usando um for) e imprima o número e qual grupo ele pertence:
//Entre 1 a 10: Grupo A
//Entre 11 a 20: Grupo B
//Entre 21 a 30: Grupo C
//Entre 31 a 40: Grupo D
//Entre 41 a 50: Grupo E
//Exemplo:
//1: Grupo A
//2: Grupo A
//3: Grupo A
//...

fn main() {

    //Criação do for nos atentando em relação ao '=' no range.  
    for i in 1..=50{
        match i {
            1..=10 => println!("{}: Grupo A", i),
            11..=20 => println!("{}: Grupo B",i),
            21..=30 => println!("{}: Grupo C",i),
            31..=40 => println!("{}: Grupo D",i),
            
            //Criação do default para conseguir tratar todos os dados.
            _ => println!("{}: Grupo E",i)
        };
    }
}