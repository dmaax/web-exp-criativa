# Guia Básico de Git

## O que é Git?

Git é um sistema de controle de versão distribuído, amplamente usado para rastrear mudanças em arquivos de código-fonte durante o desenvolvimento de software. Ele permite colaboração eficiente entre desenvolvedores, mantendo um histórico de alterações.

## Configuração Inicial

Antes de começar a usar o Git, é importante configurá-lo:

```bash
# Configurar o nome de usuário
git config --global user.name "Seu Nome"

# Configurar o email do usuário
git config --global user.email "seu.email@exemplo.com"

# Verificar as configurações
git config --list
```

## Principais Comandos do Git

### Inicializar um Repositório

Cria um novo repositório Git em um diretório:

```bash
git init
```

### Clonar um Repositório

Copia um repositório remoto para o computador local:

```bash
git clone <url-do-repositorio>
```

Exemplo para copiar esse repositório:
```bash
git clone https://github.com/dmaax/web-exp-criativa.git
```

### Adicionar Arquivos ao Staging

Adiciona mudanças de arquivos específicos à área de staging:

```bash
git add <arquivo>

# Adiciona todas as mudanças
git add .
```

### Realizar um Commit

Registra as mudanças na área de staging com uma mensagem descritiva:

```bash
git commit -m "Mensagem descritiva das mudanças"
```

### Verificar o Status

Mostra o estado atual do repositório:

```bash
git status
```

### Verificar o Histórico de Commits

Exibe o histórico de commits:

```bash
git log
```

### Criar e Mudar de Branch

Cria uma nova branch:

```bash
git branch <nome-da-branch>
```

Troca para uma branch existente:

```bash
git checkout <nome-da-branch>
```

Criar uma nova branch e já trocar para ela:

```bash
git checkout -b <nome-da-branch>
```

### Enviar Mudanças para o Repositório Remoto

Envia os commits locais para o repositório remoto:

```bash
git push origin <nome-da-branch>
```

### Atualizar o Repositório Local

Baixa as mudanças do repositório remoto:

```bash
git pull origin <nome-da-branch>
```

### Mesclar Branches

Une uma branch específica à branch atual:

```bash
git merge <nome-da-branch>
```

### Resolver Conflitos

Quando há conflitos em um merge, o Git marca as seções conflitantes nos arquivos afetados. Resolva manualmente e depois:

```bash
git add <arquivo-resolvido>
git commit -m "Resolvendo conflito"
```

## Conclusão

Este é um guia básico para começar a usar o Git. Ele cobre os comandos mais comuns para gerenciar seu código-fonte e colaborar em projetos. Para mais detalhes, consulte a [documentação oficial do Git](https://git-scm.com/doc).


