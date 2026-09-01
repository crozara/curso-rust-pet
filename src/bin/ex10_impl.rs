//Exercício 10 - Blocos de implementação
//-----------------------------------------------------------------
///Na struct Disciplina criada, crie as seguintes funções:
///new (é um constructor) - para criar uma disciplina mais fácil
///mostrar_info - para imprimir toda as informações de uma disciplina
///trocar_prof - para alterar permanentemente o prof da disciplina

struct Disciplina {
    _nome: String,
    _num_sala: i16,
    _qtd_alunos: i8,
    _nome_professor: String
}

impl Disciplina {

    //constructor
    fn new(nome: &str, num_sala: i16, qtd_alunos: i8, nome_professor: &str) -> Disciplina {
        Disciplina {
            _nome: String::from(nome),
            _num_sala: num_sala,
            _qtd_alunos: qtd_alunos,
            _nome_professor: String::from(nome_professor)
        }
    }
    
    fn mostrar_info(&self) {
        println!("Nome da disciplina: {}", self._nome);
        println!("Número da sala: {}", self._num_sala);
        println!("Quantidade de alunos: {}", self._qtd_alunos);
        println!("Nome do professor: {}", self._nome_professor);
    }

    fn trocar_prof(&mut self, novo_nome: &str){
        self._nome_professor = String::from(novo_nome)
    }
}

fn main() {
    //testes com nossas disciplinas
    let calculo: Disciplina = Disciplina::new("Calculo Diferencial e Integral I", 404, 30, "Leandro");
    let mut geometria: Disciplina = Disciplina::new("Geometria Analítica", 404, 35, "Jorge");
    calculo.mostrar_info();
    geometria.mostrar_info();
    geometria.trocar_prof("Paulo");
    geometria.mostrar_info();
}
