// https://github.com/andrelmlins/cpf_cnpj

use rocket::{http::Status, serde::json::Json};

#[derive(serde::Deserialize)]
pub struct CpfConf {
    pub cpf: String,
}

#[derive(serde::Serialize)]
pub struct CpfResponse {
    pub valido: bool,
}

pub fn validate(valor: &str) -> bool {
    let numbers = valor
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    if numbers.len() != 11 || equal_digits(&numbers) {
        return false; 
    }
    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[9].to_string().parse::<usize>().unwrap() {
        return false;
    }
    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[10].to_string().parse::<usize>().unwrap() {
        return false;
    }
    return true;
}

fn validate_first_digit(numbers: &Vec<char>) -> usize {
    let mut count = 10;
    let mut _sum = 0;
    for number in numbers {
        if count >= 2 {
            let number_int = number.to_string().parse::<usize>().unwrap();
            _sum += number_int * count;
            count -= 1;
        }
    }
    let result = (_sum * 10) % 11;

    if result == 10 {
        return 0;
    } else {
        return result;
    }
}

fn validate_second_digit(numbers: &Vec<char>) -> usize {
    let mut count = 11;
    let mut _sum = 0;
    for number in numbers {
        if count >= 2 {
            let number_int = number.to_string().parse::<usize>().unwrap();
            _sum += number_int * count;
            count -= 1;
        }
    }
    let result = (_sum * 10) % 11;

    if result == 10 {
        return 0;
    } else {
        return result;
    }
}
fn equal_digits(numbers: &Vec<char>) -> bool {
    let mut digit = String::from("");
    for number in numbers {
        if digit == "" {
            digit = number.to_string();
        } else if digit != number.to_string() {
            return false;
        }
    }
    return true;
}
// simples, recebe o cpf e devolve se é valido ou nao
#[post("/verifica_cpf", format = "json", data = "<cpf_data>")]
pub async fn vcpf(cpf_data: Json<CpfConf>) -> Result<Json<CpfResponse>, Status> {
    let valido: bool = validate(&cpf_data.cpf);
    Ok(Json(CpfResponse { valido }))
}
