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

    // Casting
    let float_thirty_two = 10.2_f32;
    let unsigned_eight: u8 = 12;
}
