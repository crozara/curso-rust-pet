// Exercicio 08 - Borrowing Mutável
//-----------------------------------------------------------------
// Faça uma função que receba uma String e altere permanentemente ela.

fn alterar_string(s: &mut String) {
    // Usamos o push_str para concatenar e alterar a String original
    s.push_str(" - modificado permanentemente!");
}

fn main() {
    // A variável precisa ser mutável para conseguirmos alterar.
    let mut texto = String::from("Texto original");
    println!("{}", texto);

    // Passamos a referência mutável
    alterar_string(&mut texto);

    println!("{}", texto);
}