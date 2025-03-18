use rocket::form::Form;

#[derive(FromForm)]
pub struct Cadastro {
    pub nome: String,
    pub email: String,
    pub cpf: String,
    pub senha: String,
    pub confirmar_senha: String,
}

// processa o cadastro
pub async fn cadastrar(cadastro: Form<Cadastro>) -> String {
    let cadastro = cadastro.into_inner();
// aqui a gente coloca a fn hash e verifica as coisas
 /* 
    if cadastro.senha != cadastro.confirmar_senha {
        return "as senhas não sao iguais ".to_string();
    }

    for l in cadastro.email{

    }
*/
    format!("Cadastro realizado com sucesso para {}", cadastro.nome)
}
