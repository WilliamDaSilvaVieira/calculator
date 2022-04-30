use std::io;

fn main() {
    println!("Bem vindo a Calculadora!");
    println!("Instruções: Coloque o número incial e depois escolha uma função.\nEsta calculadora tem as funções de 'Adição', 'Subtração', 'Multiplicação', 'Divisão' e 'Porcentagem'\nPara números com virgula utilizem o ponto('.').\nPara finalizar a calculadora e mostrar o resultado digite 'resultado'.");
    let mut numero: f64 = loop {
        println!("Digite o número desejado!");
        let mut numero = String::new();
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao obter o número.");
        match numero.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };
    let resultado: f64 = loop {
        let operacao = loop {
            println!("Que tipo de operação você que fazer com {}?", numero);
            let mut x = String::new();
            io::stdin()
                .read_line(&mut x)
                .expect("Falha ao processar operação!");
            match x.to_lowercase().trim() {
                "adição" | "subtração" | "multiplicação" | "divisão" | "porcentagem"
                | "resultado" => break x.trim().to_lowercase(),
                _ => {
                    println!("Operação não existe! Verifique os acentos e ç onde tiver!");
                    continue;
                }
            }
        };
        match operacao.trim() {
            "adição" => numero = adicao(numero),
            "subtração" => numero = subtracao(numero),
            "multiplicação" => numero = multiplicacao(numero),
            "divisão" => numero = divisao(numero),
            "porcentagem" => numero = porcentagem(numero),
            _ => break numero,
        }
    };
    println!("O resultado final é: {}", resultado);
}

fn adicao(n: f64) -> f64 {
    n + num(n, "somar")
}

fn subtracao(n: f64) -> f64 {
    n - num(n, "subtrair")
}

fn multiplicacao(n: f64) -> f64 {
    n * num(n, "multiplicar")
}

fn divisao(n: f64) -> f64 {
    n / num(n, "dividir")
}

fn porcentagem(n: f64) -> f64 {
    n * num(n, "porcentagem") / 100.0
}

fn num(n: f64, tipo: &str) -> f64 {
    let num: f64 = loop {
        if tipo == "porcentagem" {
            println!("Qual {} você quer de {}?", tipo, n);
        } else {
            println!("Quer {} {} com qual número?", tipo, n);
        }
        let mut x = String::new();
        io::stdin()
            .read_line(&mut x)
            .expect("Não foi possivel ler.");
        match x.trim().parse() {
            Ok(num) => {
                if tipo == "dividir" {
                    if num != 0.0 {
                        break num;
                    } else {
                        println!("Impossivel dividir por zero!");
                        continue;
                    }
                } else {
                    break num;
                }
            }
            Err(_) => {
                println!("Digite somente números!");
                continue;
            }
        }
    };
    num
}
