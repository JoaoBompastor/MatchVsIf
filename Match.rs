use std::str::FromStr;

fn main() {
    loop {
        println!("
1) somar dois numeros
2) multiplicar dois numeros
3) dividir dois numeros
4) fechar programa
        ");
        
        let escolha: i32 = ler_numero();

        match escolha {
            1 => {
                println!("Digite o primeiro numero da soma: ");
                let primeiro_num: i32 = ler_numero();

                println!("Agora, digite o segundo numero da soma: ");
                let segundo_num: i32 = ler_numero();

                println!("A soma entre {} e {} é {}", primeiro_num, segundo_num,
                somar(primeiro_num, segundo_num))
            }

            2 => {
                println!("Digite o primeiro numero da multiplicação: ");
                let primeiro_num: i32 = ler_numero();

                println!("Agora, digite o segundo numero da multiplicação: ");
                let segundo_num: i32 = ler_numero();

                println!("A multiplicação entre {} e {} é {}", primeiro_num, segundo_num,
                multiplicar(primeiro_num, segundo_num));
            }

            3 => {
                println!("Digite o primeiro mumero para a divisão: ");
                let primeiro_num: i32 = ler_numero();

                println!("Agora, digite o segundo numero para a divisão: ");
                let segundo_num: i32 = ler_numero();

                println!("A divisão entre {} e {} é {}", primeiro_num, segundo_num,
                dividir(primeiro_num, segundo_num));
            }

            4 => {
                break
            }

            _ => {
                println!("Opção inválida. Tente novamente.")
            }
        }
    }
}

fn somar(primeiro_num: i32, segundo_num: i32) -> i32 {
    primeiro_num + segundo_num
}

fn multiplicar(primeiro_num: i32, segundo_num: i32) -> i32 {
    primeiro_num * segundo_num
}

fn dividir(primeiro_num: i32, segundo_num: i32) -> i32 {
    primeiro_num / segundo_num
}

fn ler_string() -> String {
    let mut input: String = String::new();

    std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
    input
        .trim()
        .to_lowercase()
}

fn ler_numero() -> i32 {
    let mut input: String = ler_string();

    i32::from_str(&mut input).expect("failed to read line.")
}
