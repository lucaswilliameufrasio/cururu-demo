# Contribuindo

## Diretrizes de revisão

- Use `Result` em vez de `unwrap()` para tratamento de erros.
- Evite `sh -c` e shell expansion. Prefira APIs seguras com argumentos em vetor.
- Valide entrada do usuário: caminhos não devem conter `..`, queries não devem
  conter metacaracteres de shell.
- Estado global compartilhado entre threads deve usar sincronização adequada
  (ex: `Arc<Mutex<T>>` para acesso exclusivo).
- Todo comando novo precisa de teste unitário correspondente.
- Prefira `write!` e `read!` seguros em vez de concatenar strings para comandos.

## Padrões de código

```rust
// Bom: usa iterator, sem unwrap
fn process(items: &[String]) -> Result<(), Error> {
    for item in items {
        item.validate()?;
    }
    Ok(())
}

// Ruim: unwrap, command injection, path traversal
fn save(path: &str) {
    std::fs::write(path, data).unwrap();
}
```
