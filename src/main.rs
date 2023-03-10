// Constantes são inlined em tempo de compilação, ou seja,
// o compilador substitui a referência para a constante pelo seu valor diretamente.
const PI: f32 = 3.1415;
static mut GLOBAL: u8 = 1;

fn main() {
    println!("O valor de PI é: {}", PI);

    // Usamos unsafe para acessar variáveis globais mutáveis.
    println!("Variavel global = {}", unsafe { GLOBAL });

    let variavel: i32 = 300;
    println!(
        "O valor da variável é: {}, tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    let decimal: f32 = 2.5;
    println!(
        "O valor de decimal é: {}, tamanho = {} bytes",
        decimal,
        std::mem::size_of_val(&decimal)
    );

    // Em Rust, todas as variáveis são imutáveis por padrão. Para poder alterar seus valores, precisamos declará-las com let mut.
    //let mut booleano: bool = true;
    let booleano: bool = true;
    println!(
        "O valor de booleano é: {}, tamanho = {} bytes",
        booleano,
        std::mem::size_of_val(&booleano)
    );

    let caracter: char = 'a';
    println!(
        "O valor de caracter é: {}, tamanho = {} bytes",
        caracter,
        std::mem::size_of_val(&caracter)
    );

    sombra();
    println!("Soma = {}", soma(10, 20));
    condicionais();
    repeticoes();
    ownership();
    pattern_matching();
    erros();
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

fn condicionais() {
    let idade = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    // Os parênteses são opcionais, mas recomendados.
    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    // Podemos usar o if como uma expressão, e atribuir seu valor a uma variável.
    // É como se o if pudesse retornar um valor, assim como funções.
    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);

    // O match é um switch case em Rust.
    // O match é uma expressão, assim como o if.
    // O match é mais poderoso que o switch case, pois podemos usar expressões booleanas.
    let linguagem = "Rust";
    match linguagem {
        "Rust" => println!("Linguagem de programação"),
        "Python" => println!("Linguagem de script"),
        "C" | "C++" => println!("Linguagem de baixo nível"),
        _ => println!("Linguagem desconhecida"),
    };
}

fn repeticoes() {
    let multiplicador: u8 = 5;

    let mut contador: u8 = 0;
    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }

        println!(
            "{} x {} = {}",
            contador,
            multiplicador,
            contador * multiplicador
        );
    }

    println!("Fim do while");

    println!("Inicio do loop");

    contador = 0;
    loop {
        contador += 1;
        println!(
            "{} x {} = {}",
            contador,
            multiplicador,
            contador * multiplicador
        );

        if contador == 10 {
            break;
        }
    }

    println!("Fim do loop");

    println!("Inicio do for");

    // Intervalos por padrão não incluem o número final. Para isso faríamos 1..=10.
    // O for também pode ser usado como uma expressão, assim como o if.
    for i in 1..=10 {
        println!("{} x {} = {}", i, multiplicador, i * multiplicador);
    }
}

fn ownership() {
    // O Rust não possui garbage collector, e sim ownership.
    // O ownership é uma forma de gerenciar a memória.
    /*
    Ao passar a referência para uma variável que aponta para um endereço na heap nós 
    não precisamos copiar o dado da heap nem mover o recurso (tornando a variável original inválida).
    */
    let mut uma_string = String::from("Gabriel");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

// O Rust não possui ponteiros, mas podemos usar referências.
// As referências são como ponteiros, mas não podem ser nulas.
// As referências são imutáveis por padrão, mas podemos usar mut para torná-las mutáveis.
// As referências são usadas para evitar a cópia de dados, e para evitar que o valor seja movido.
fn rouba(string_roubada: &mut String) {
    string_roubada.push_str(" Vieira");
    println!("{}", string_roubada);
}


fn pattern_matching() {
    // O pattern matching é uma forma de fazer comparações.
    // Podemos usar o pattern matching para comparar valores, e também para desestruturar valores.

    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade Par",
            _ => "Muuuuuito"
        });
    }
}
  
fn erros() {
    match resultado() {
        Ok(valor) => println!("Valor: {}", valor),
        Err(erro) => println!("Erro: {}", erro),
    }
    // O panic! é usado para parar a execução do programa.
    // Stacktrace é impresso no console.
    // Usado para erros que não podem ser tratados. ex: Erro de conexão com o banco de dados.
    // Erro que não é recuperável.
    panic!("Erro proposital");
}

fn resultado() -> Result<i32, String> {
    let valor = 10;
    if valor > 0 {
        // O Result é um enum que pode ter dois valores, Ok ou Err.
        // O Ok é usado para retornar um valor, e o Err é usado para retornar um erro.
        Ok(valor)
    } else {
        Err(String::from("Valor inválido"))
    }
}

