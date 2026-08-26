//Exercicio 03 - Loops - Resolução utilizando if e else.
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
    for i in 1..=50 {
        if i >=1 && i <=10 {
            println!("{}: Grupo A", i);
        }
        else if i >= 11 && i <=20 {
            println!("{}: Grupo B", i);
        }
        else if i >= 21 && i <=30 {
            println!("{}: Grupo C", i);
        }
        else if i >= 31 && i <=40 {
            println!("{}: Grupo D", i);
        }
        else if i >= 41 && i <=50 {
            println!("{}: Grupo E", i);
        }
    }
}