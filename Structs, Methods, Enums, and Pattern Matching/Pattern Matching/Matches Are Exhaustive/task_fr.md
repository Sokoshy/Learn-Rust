### Les correspondances sont exhaustives

Il y a un autre aspect de `match` que nous devons aborder. Considérons cette version de notre fonction `plus_one` qui contient un bug et ne se compilera pas :

```rust,ignore,does_not_compile
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

Nous n'avons pas traité le cas `None`, donc ce code entraînera un bug. Heureusement, c'est un bug que Rust sait attraper. Si nous essayons de compiler ce code, nous obtiendrons cette erreur :

```console
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: assurez-vous que tous les cas possibles sont traités, éventuellement en ajoutant des jokers ou plus de branches de correspondance
    = note: la valeur correspondante est de type `Option<i32>`
```

Rust sait que nous n'avons pas couvert tous les cas possibles et sait même quel motif nous avons oublié ! Les correspondances dans Rust sont *exhaustives* : nous devons épuiser toutes les possibilités pour que le code soit valide. Surtout dans le cas de `Option<T>`, quand Rust nous empêche d'oublier de gérer explicitement le cas `None`, il nous protège de supposer que nous avons une valeur alors que nous pourrions avoir un nul, rendant ainsi impossible l'erreur d'un milliard de dollars évoquée précédemment.