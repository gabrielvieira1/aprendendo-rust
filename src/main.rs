// Constantes são inlined em tempo de compilação, ou seja, 
// o compilador substitui a referência para a constante pelo seu valor diretamente.
const PI:f32 = 3.1415;
static mut GLOBAL:u8 = 1;

fn main() {
    println!("O valor de PI é: {}", PI);


    // Usamos unsafe para acessar variáveis globais mutáveis.
    println!("Variavel global = {}", unsafe { GLOBAL });


    let variavel: i32 = 300;
    println!("O valor da variável é: {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal: f32 = 2.5;
    println!("O valor de decimal é: {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    // Em Rust, todas as variáveis são imutáveis por padrão. Para poder alterar seus valores, precisamos declará-las com let mut.
    let mut booleano: bool = true;
    booleano = false;
    println!("O valor de booleano é: {}, tamanho = {} bytes", booleano, std::mem::size_of_val(&booleano));

    let caracter: char = 'a';
    println!("O valor de caracter é: {}, tamanho = {} bytes", caracter, std::mem::size_of_val(&caracter));

    sombra();
    println!("Soma = {}", soma(10, 20));
}
// Shadowing
fn sombra() {
    println!("Inicio da função sombra");

    let a = 123;

    // Temos o contexto de shadowing, onde podemos declarar uma variável com o mesmo nome de uma variável já declarada.
    // Nesse caso, a variável A é declarada novamente, mas com um tipo diferente.
    // Isso pode ser periloso, pois podemos acabar sobrescrevendo uma variável sem querer.
    {
        let b = 456;
        println!("dentro b = {}", b);
        
        let a = 777;
        println!("dentro a = {}", a);
    }

    println!("fora a = {}", a);
}

fn soma(a: i32, b: i32) -> i32 {
    println!("A soma de {} + {} é: {}", a, b, a + b);
    // Em rust tudo é uma expressão, ou seja, podemos retornar o valor de uma expressão.
    // Nesse caso, retornamos o valor da soma de a + b.
    // Se não definirmos o retorno, a função não poderá retornar um valor. 
    // Além disso, a última expressão não deve conter ; para que seu valor seja usado como retorno.
    a + b
}



