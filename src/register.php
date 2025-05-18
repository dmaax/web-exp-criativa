<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <?php

if ($_SERVER["REQUEST_METHOD"] == "POST"){
    $nomeCompleto = $_POST['name'] ?? '';

    header("Location: ../static/html/conf_email.html");
    exit;
} else {
    header("Location: ../static/html/register_page.html");
    exit;
}

?>
    
</body>
</html>
