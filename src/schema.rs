// @generated automatically by Diesel CLI.

diesel::table! {
    usuarios (id) {
        id -> Int4,
        #[max_length = 100]
        nome -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 12]
        cpf -> Varchar,
        #[max_length = 10]
        data_nascimento -> Varchar,
        #[max_length = 16]
        telefone -> Varchar,
        senha_hash -> Text,
        #[max_length = 9]
        cep -> Varchar,
        #[max_length = 32]
        codigo_2fa -> Varchar,
    }
}
