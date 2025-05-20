<?php

require __DIR__ . '/funcoes-php/funcao-registro.php';

//Confere se chega o metodo POST, cas nao chegue ele redireciona para pagina de registro;
if ($_SERVER["REQUEST_METHOD"] == "POST"){

    //Atribuindo valore a variaveis
    //A funcao htmlspecialchars evita injection
    $nomeCompleto = htmlspecialchars($_POST["name"]);
    $cpf = htmlspecialchars($_POST["cpf"]);
    $dataNasc = htmlspecialchars($_POST["birthdate"]);
    $email = htmlspecialchars($_POST["email"]);
    $celular = htmlspecialchars($_POST["cellphone"]);
    $cep = htmlspecialchars($_POST["cep"]);
    $senha = htmlspecialchars($_POST["password"]);
    $senhaConf = htmlspecialchars($_POST["confirmPassword"]);

    //Confere se campo esta vazio
    //Lembrar de fazer instancia para cada campo
    if (empty($nomeCompleto)){
        die("Nome necessario");
    } if (empty($cpf)){
        exit();
    } if (empty($dataNasc)){
        exit();
    } if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
        exit();
    } if (empty($celular)) {
        exit();
    } if (empty($cep)) {
        exit();
    } if (empty($senha)) {
        exit();
    } if (empty($senhaConf)) {
        exit();
    } 

    
    //Checando senha
    if ( ! validarSenha($senha, $senhaConf)){
        exit();
    }
    //Hashing da senha
    $senhaHash = password_hash ($senha, PASSWORD_DEFAULT);

    //Conexao SQL
    $mysqli = require "../../src/database.php";


    //Insercao de dados no banco DB
    $sql = "INSERT INTO user (name, cpf, email, celular, cep, senha, data_nasc)
            VALUES (?, ?, ?, ?, ?, ?, ? )";
    
    $stmt = $mysqli->stmt_init();
    if (! $stmt-> prepare($sql)) {
        die("SQL erro: " . $mysqli->error);
    }
    $stmt->bind_param("sssssss",
                        $nomeCompleto,
                        $cpf,
                        $email,
                        $celular,
                        $cep,
                        $senhaHash,
                        $dataNasc);

    $stmt->execute();

    die("Suceesso");
    header("Location: ../static/html/login_");
} else {
    header("Location: ../static/html/register_page.html");
    exit;
}
