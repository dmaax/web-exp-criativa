// @generated automatically by Diesel CLI.

diesel::table! {
    cartoes (id) {
        id -> Int4,
        conta_id -> Int4,
        #[max_length = 30]
        numero_cartao -> Varchar,
        #[max_length = 8]
        data_cartao -> Varchar,
        #[max_length = 3]
        codigo_cartao -> Varchar,
        #[max_length = 20]
        saldo_disponivel -> Varchar,
        #[max_length = 20]
        saldo_usado -> Varchar,
    }
}

diesel::table! {
    contas (id) {
        id -> Int4,
        usuario_id -> Int4,
        #[max_length = 20]
        saldo -> Varchar,
    }
}

diesel::table! {
    emprestimos (id) {
        id -> Int4,
        conta_id -> Int4,
        #[max_length = 20]
        valor_disponivel -> Varchar,
        #[max_length = 20]
        valor_emprestado -> Varchar,
    }
}

diesel::table! {
    extratos (id) {
        id -> Int4,
        conta_id -> Int4,
        #[max_length = 100]
        nome_compra -> Varchar,
        #[max_length = 20]
        valor -> Varchar,
    }
}

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

diesel::joinable!(cartoes -> contas (conta_id));
diesel::joinable!(contas -> usuarios (usuario_id));
diesel::joinable!(emprestimos -> contas (conta_id));
diesel::joinable!(extratos -> contas (conta_id));

diesel::allow_tables_to_appear_in_same_query!(
    cartoes,
    contas,
    emprestimos,
    extratos,
    usuarios,
);
