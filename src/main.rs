#![allow(unused_variables)]

fn main() {
    // Variaveis em Rust são imutaveis por padrão
    // Declaração de variavel padrão
    let variable: u32 = 34;
    let variable = 34_u64;

    // Inferencia de tipo
    let variable = 3.4;

    // Variavel não utilizada, compilador não reclama
    let _variable_nao_ultilizada = 70;

    // Casting, Rust não faz casting automatico
    let float_thirty_two = 10.2_f32;
    let unsigned_eight: u8 = 12;
    let cast_unsigned_eight = unsigned_eight as f32;
    let sum = float_thirty_two + cast_unsigned_eight;
    println!("{}", sum);

    // cast int(não pode ser float, assim como um "u128) to char
    let int_char = 65_u8;
    let cast_int_char = int_char as char;
    println!("{}", cast_int_char);

    // Variavel mutavel using (mut)
    let mut variable_mutavel = 10;

    // Shadowing and scope

}
