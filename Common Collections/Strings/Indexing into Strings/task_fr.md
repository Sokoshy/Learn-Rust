### Indexation dans les chaînes de caractères

Dans de nombreux autres langages de programmation, accéder à des caractères individuels d'une chaîne en les référenciant par leur indice est une opération valide et courante. Cependant, si vous essayez d'accéder à des parties d'une `String` en utilisant la syntaxe d'indexation en Rust, vous obtiendrez une erreur. Considérez le code invalide ci-dessous.

```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

##### Tentative d'utilisation de la syntaxe d'indexation avec une String

Ce code entraînera l'erreur suivante :

```text
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
```

L'erreur et la note racontent l'histoire : les chaînes de caractères Rust ne prennent pas en charge l'indexation. Mais pourquoi ? Pour répondre à cette question, nous devons discuter de la manière dont Rust stocke les chaînes en mémoire.
Vous pouvez lire les détails dans [Le Livre](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#indexing-into-strings).