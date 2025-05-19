<?php

$host = "localhost";
$dbname = "login_db";
$username = "root";
$password = "";

$mysqli = new mmysqli($host, $username, $username, $dbname);

if ($mysqli-> connect_errno){
    die("Erro de conexao com banco de dados: " . $mysqli->connect_error);
}

return $mysqli;