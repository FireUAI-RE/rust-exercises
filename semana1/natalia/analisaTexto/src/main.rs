use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    match executar() {
        Ok(()) => ExitCode::SUCCESS,
        Err(mensagem) => {
            eprintln!("Erro: {mensagem}");
            ExitCode::FAILURE
        }
    }
}

fn executar() -> Result<(), String> {
    println!("entrada:");
    let texto = ler_linha()?;
    let procurada = ler_palavra_procurada()?;

    let relatorio = analisar(&texto, &procurada);
    imprimir_relatorio(&relatorio, &procurada);

    Ok(())
}

fn imprimir_relatorio(relatorio: &Relatorio, procurada: &str) {
    println!();
    println!("saída:");
    println!("caracteres: {}", relatorio.caracteres);
    println!("palavras: {}", relatorio.palavras);
    match relatorio.maior_palavra {
        Some(palavra) => println!("maior palavra = {palavra}"),
        None => println!("maior palavra = (nenhuma)"),
    }
    println!("ocorrências de {procurada}: {}", relatorio.ocorrencias);
}

fn ler_linha() -> Result<String, String> {
    let mut linha = String::new();

    match io::stdin().read_line(&mut linha) {
        Ok(0) => Err(String::from("a entrada terminou antes do esperado")),
        Ok(_) => {
            let tamanho = linha.trim_end_matches(['\r', '\n']).len();
            linha.truncate(tamanho);
            Ok(linha)
        }
        Err(erro) => Err(format!("não foi possível ler a entrada ({erro})")),
    }
}
fn ler_palavra_procurada() -> Result<String, String> {
    loop {
        print!("palavra procurada: ");
        io::stdout()
            .flush()
            .map_err(|erro| format!("não foi possível escrever na tela ({erro})"))?;

        let linha = ler_linha()?;
        match validar_palavra(&linha) {
            Ok(palavra) => return Ok(palavra.to_string()),
            Err(motivo) => println!("Entrada inválida: {motivo}. Tente novamente."),
        }
    }
}
fn validar_palavra(entrada: &str) -> Result<&str, &'static str> {
    let mut tokens = entrada.split_whitespace();
    let primeira = tokens.next().ok_or("nenhuma palavra foi digitada")?;
    if tokens.next().is_some() {
        return Err("digite apenas uma palavra");
    }

    let palavra = limpar_palavra(primeira);
    if palavra.is_empty() {
        return Err("a palavra precisa ter ao menos uma letra ou número");
    }

    Ok(palavra)
}
#[derive(Debug, PartialEq, Eq)]
struct Relatorio<'a> {
    caracteres: usize,
    palavras: usize,
    maior_palavra: Option<&'a str>,
    ocorrencias: usize,
}
fn analisar<'a>(texto: &'a str, procurada: &str) -> Relatorio<'a> {
    Relatorio {
        caracteres: contar_caracteres(texto),
        palavras: contar_palavras(texto),
        maior_palavra: maior_palavra(texto),
        ocorrencias: contar_ocorrencias(texto, procurada),
    }
}
fn contar_caracteres(texto: &str) -> usize {
    texto.chars().count()
}
fn contar_palavras(texto: &str) -> usize {
    let mut total = 0;
    for token in texto.split_whitespace() {
        if !limpar_palavra(token).is_empty() {
            total += 1;
        }
    }
    total
}
fn maior_palavra(texto: &str) -> Option<&str> {
    let mut maior = None;
    let mut tamanho_maior = 0;

    for token in texto.split_whitespace() {
        let palavra = limpar_palavra(token);
        let tamanho = contar_caracteres(palavra);
        if tamanho > 0 && tamanho >= tamanho_maior {
            maior = Some(palavra);
            tamanho_maior = tamanho;
        }
    }

    maior
}
fn contar_ocorrencias(texto: &str, procurada: &str) -> usize {
    let alvo = limpar_palavra(procurada).to_lowercase();
    if alvo.is_empty() {
        return 0;
    }

    let mut total = 0;
    for token in texto.split_whitespace() {
        if limpar_palavra(token).to_lowercase() == alvo {
            total += 1;
        }
    }
    total
}
fn limpar_palavra(token: &str) -> &str {
    token.trim_matches(|c: char| !c.is_alphanumeric())
}
