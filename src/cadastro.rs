use rocket::form::Form;
use bcrypt::{hash, DEFAULT_COST};

#[derive(FromForm)]
pub struct Cadastro {
    pub nome: String,
    pub email: String,
    pub cpf: String,
    pub data_nascimento: String,
    pub telefone: String,
    pub senha: String,
    pub confirmar_senha: String,
    pub rua: String,
    pub numero: String,
    pub complemento: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub cep: String,

}
fn valida_email(email: &str) -> bool{
    email.contains("@") && email.contains(".")
}

fn hash_senha(senha: &str) -> String{
    hash(senha, DEFAULT_COST).unwrap()
}



// processa o cadastro
pub async fn cadastrar(cadastro: Form<Cadastro>) -> String {
    let cadastro = cadastro.into_inner();

    if !valida_email(&cadastro.email) {
        return "Email inválido".to_string();
    }

    if cadastro.senha != cadastro.confirmar_senha {
        return "As senhas não são iguais".to_string();
    }

    // https://github.com/andrelmlins/cpf_cnpj
    
    let senha_hash = hash_senha(&cadastro.senha);

    format!("Cadastro realizado com sucesso para {}", cadastro.nome)
}
