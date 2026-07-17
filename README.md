# cururu-demo

Repositório de demonstração para [Cururu](https://github.com/lucaswilliameufrasio/cururu),
um bot de revisão de PRs baseado em LLM.

**Atenção:** Este repositório contém intencionalmente código com falhas de
segurança, concorrência e manutenibilidade para testar a capacidade de análise
do Cururu.

## Uso

```bash
cargo run -- greet Alice Bob
cargo run -- save notas.txt "conteudo seguro"
cargo run -- search padrao
cargo run -- admin
```

## Comandos

| Comando | Descrição |
|---------|-----------|
| `greet` | Saúda nomes fornecidos |
| `save` | Salva conteúdo em arquivo |
| `search` | Busca texto nos dados |
| `admin` | Modo administrativo |
| `process` | Processa itens em lote |

## Licença

MIT
