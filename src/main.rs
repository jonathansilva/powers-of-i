use std::io;

fn powers(number: i32) -> String {
    match number {
        0 => "1".to_string(),
        1 => "i".to_string(),
        2 => "-1".to_string(),
        3 => "-i".to_string(),
        -1 => "-i".to_string(),
        -2 => "1".to_string(),
        -3 => "i".to_string(),
        _ => unreachable!()
    }
}

fn to_superscript(number: &str) -> String {
    number.chars()
        .map(|c: char| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => c
        }).collect()
}

fn main() {
    let mut input: String = String::new();

    println!("\nDigite um número e pressione Enter\n");

    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

    let number: i32 = match input.trim().parse() {
        Ok (number) => number,
        Err (_) => {
            println!("\nInforme um número inteiro\n");

            return;
        }
    };

    let modulus: i32 = number % 4;

    let power: String = powers(modulus);

    let i_with_exponent: String = format!("i{}", to_superscript(input.trim()));

    let message: String = format!(
        "\n{number} dividido por 4 possui resto {modulus}\n\
        Resto {modulus} equivale à {power}\n\
        {i_with_exponent} = {power}"
    );

    println!("{message}");

    println!("\n{:-^25}", " Powers of i ");
    println!("Powers   | Calculation");
    println!("{}", "-".repeat(25));
    println!("0  =>  1 | {i_with_exponent} = ?");
    println!("1  =>  i | x = {number} % 4");
    println!("2  => -1 | x = {modulus} => {power}");
    println!("3  => -i | {i_with_exponent} = {power}");
    println!("{}\n", "-".repeat(25));
}
