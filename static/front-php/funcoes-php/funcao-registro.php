<?php
// Funcao que valida a senha
function validarSenha(string $senha, string $senhaConf): bool{
    if (strlen($senha) < 8){
        die("Senha deve conter 8 caracteres");
        return false;
    }
    if (! preg_match("/[0-9]/", $senha)){
        die("Senha precisa conter numeros");
        return false;
    }
    if (! preg_match("/[a-z]/", $senha)){
        die("Senha precisa conter letras minusculas");
        return false;
    }
    if (! preg_match("/[A-Z]/", $senha)){
        die("Senha precisa conter letras maiusculas");
        return false;
    }
    if (! preg_match('/[^a-zA-Z0-9]/', $senha)){
        die("Precisa conter caracteres especiais");
        return false;
    }
    if ($senha !== $senhaConf){
        die("Senhas devem coincidir");
        return false;
    }
    return true;
}

