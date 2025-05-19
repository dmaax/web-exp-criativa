<?php

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




} else {
    header("Location: ../static/html/register_page.html");
    exit;
}
