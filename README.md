# cururu-demo

> [!CAUTION]
> # 🚨 ALERTA: DEMONSTRAÇÃO INTENCIONALMENTE INSEGURA — NÃO USE EM PRODUÇÃO 🚨
>
> Este repositório contém vulnerabilidades reais e deliberadas de injeção de
> comandos, traversal de caminhos, autorização, concorrência e acesso a rede.
> Os exemplos existem exclusivamente para demonstrar o Cururu v4.9.0 e podem
> executar operações perigosas. **Não copie, implante nem execute este código
> fora de um ambiente descartável e controlado.** Falhas nos checks e achados
> da revisão são esperados.

Repositório de demonstração para [Cururu](https://github.com/lucaswilliameufrasio/cururu),
um bot de revisão de PRs baseado em LLM.

O workflow de revisão usa Cururu **v4.9.0**, fixado pelo commit publicado cujo `action.yml`
aponta para a imagem OCI pelo digest `sha256:0ad52d815d64e693b13a91fe09f8e375393a3fd80f371e72a241031a1b9d82c7`. Em eventos de pull request, ele também passa
`expected_head_sha` com o SHA recebido no evento: se o head mudar durante a análise,
Cururu interrompe a publicação em vez de associar os achados ao commit novo. Os
comentários inline exibem a severidade em negrito (por exemplo, **HIGH**). A
configuração habilita o manifest de ciclo de vida dos analisadores, ingere as
anotações do check run `demo-linter` e ativa a síntese/deduplicação dos achados.
O `.cururu.toml` documenta `review.recommendations = false`: com `true`, Cururu
solicita uma recomendação LLM adicional somente quando a resposta do review atinge
o limite de saída, podendo gerar custo adicional. O showcase mantém a opção desativada.

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
