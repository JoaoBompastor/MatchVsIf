use std::str::FromStr;

fn main() {
    loop {
        println!("
1) adicionar
2) multiplicar
3) dividir
4) fechar programa
        ");

        let usuario: i32 = ler_numero();

        if usuario == 1 {
            println!("Digite o primeiro numero da soma: ");
            let primeiro_numero: i32 = ler_numero();

            println!("Agora, digite o segundo numero da soma: ");
            let segundo_numero: i32 = ler_numero();

            println!("a soma entre {} e {} é {}", primeiro_numero, segundo_numero,
            somar(primeiro_numero, segundo_numero));
        }

        if usuario == 2 {
            println!("Digite o primeiro numero para a multiplicação: ");
            let primeiro_numero: i32 = ler_numero();

            println!("Agora, digite o segundo numero para a multiplicação: ");
            let segundo_numero: i32 = ler_numero();

            println!("A multiplicação entre {} e {} é {}", primeiro_numero, segundo_numero,
            multiplicar(primeiro_numero, segundo_numero));
        }

        if usuario == 3 {
            println!("Digite o primeiro numero para a divisão: ");
            let primeiro_numero: i32 = ler_numero();

            println!("Agora, digite o segundo numero para a divisão: ");
            let segundo_numero: i32 = ler_numero();

            println!("A divisao entre {} e {} é {}", primeiro_numero, segundo_numero,
            dividir(primeiro_numero, segundo_numero));
        }

        if usuario == 4 {
            break
        }

        else {
            println!("Opção inválida, tente novamente.");
        }
    }
}

fn somar(primeiro_numero: i32, segundo_numero: i32) -> i32 {
    primeiro_numero + segundo_numero
}

fn multiplicar(primeiro_numero: i32, segundo_numero: i32) -> i32 {
    primeiro_numero * segundo_numero
}

fn dividir(primeiro_numero: i32, segundo_numero: i32) -> i32 {
    primeiro_numero / segundo_numero
}

fn ler_string() -> String {
    let mut input: String = String::new();

    std::io::stdin()
            .read_line(&mut input)
            .expect("falha ao ler linha!");
    input
        .trim()
        .to_lowercase()
        .to_string()
}

fn ler_numero() -> i32 {
    let mut input: String = ler_string();

    i32::from_str(&mut input).expect("falha ao ler linha!")
}
