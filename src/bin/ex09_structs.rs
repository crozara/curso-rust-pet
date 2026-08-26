// Exercicio 09 - Structs
//-----------------------------------------------------------------
//Crie uma struct Disciplina contendo:
//Nome da disciplina
//Número da sala
//Quantidade de alunos
//Nome do professor
//E depois crie as disciplinas de Cálculo I e Geometria Analítica na função main, e por fim, imprima essas disciplinas.

//Utilizamos essa linha de código para formatar a impressão:
#[derive(Debug)]
struct Disciplina {
    nome_disciplina: String,
    num_sala: i16,
    qtde_alunos: i8,
    nome_professor: String
}

fn main() {
    let c1: Disciplina  = Disciplina {
        nome_disciplina: String::from("Calculo I"),
        num_sala: 10,
        qtde_alunos: 20,
        nome_professor: String::from("Leandro")
    };

    let ga: Disciplina  = Disciplina {
        nome_disciplina: String::from("Geometria Analitica"),
        num_sala: 10,
        qtde_alunos: 20,
        nome_professor: String::from("Jorge")
    };

    println!("{:?}", c1);
    println!("{:?}", ga);
}