# cururu-demo

> [!CAUTION]
> # 🚨 ALERTA: DEMONSTRAÇÃO INTENCIONALMENTE INSEGURA — NÃO USE EM PRODUÇÃO 🚨
>
> Este repositório contém vulnerabilidades reais e deliberadas de injeção de
> comandos, traversal de caminhos, autorização, concorrência e acesso a rede.
> Os exemplos existem exclusivamente para demonstrar o Cururu v4.8.0 e podem
> executar operações perigosas. **Não copie, implante nem execute este código
> fora de um ambiente descartável e controlado.** Falhas nos checks e achados
> da revisão são esperados.

Repositório de demonstração para [Cururu](https://github.com/lucaswilliameufrasio/cururu),
um bot de revisão de PRs baseado em LLM.

O workflow de revisão usa Cururu **v4.8.0**, fixado pelo commit cujo `action.yml`
aponta para a imagem OCI por digest. Em eventos de pull request, ele também passa
`expected_head_sha` com o SHA recebido no evento: se o head mudar durante a análise,
Cururu interrompe a publicação em vez de associar os achados ao commit novo. Os
comentários inline exibem a severidade em negrito (por exemplo, **HIGH**). A
configuração habilita o manifest de ciclo de vida dos analisadores, ingere as
anotações do check run `demo-linter` e ativa a síntese/deduplicação dos achados.

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
