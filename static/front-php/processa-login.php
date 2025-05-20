<?php
//confere se chegou post
if ($_SERVER["REQUEST_METHOD"] == "POST"){
   
} else{
    die("Erro no POST");
    exit();
}
//Pega o valor do user no login
$userId = $_POST["userId"] ?? null; 
$senhaLogin = $_POST["password"] ?? null;

//Conferencia de se usuario esta correto ou nao;
if (empty($userId)){
    die("Campo Login vazio");
}
if (empty($senhaLogin)){
    die("Campo Senha vazio");
}



//Conecta ao banco de dados
$mysqli = require "../../src/database.php";

//Confere se o campo e email ou userId
if (filter_var($userId, FILTER_VALIDATE_EMAIL)) {
    // É e-mail
    $sql = sprintf("SELECT * FROM user
                            WHERE email = '%s'",
                            //real_escape_string evita injection SQL
                            $mysqli->real_escape_string($_POST["userId"]));
} else {
    // É nome de usuário
    $sql = sprintf("SELECT * FROM user
                            WHERE name = '%s'",
                            //real_escape_string evita injection SQL
                            $mysqli->real_escape_string($_POST["userId"]));
}
   
    
$result = $mysqli->query($sql);
$user = $result->fetch_assoc(); 



var_dump($user);
exit;