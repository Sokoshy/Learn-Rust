## Tranches de chaînes comme paramètres

Savoir que vous pouvez prendre des tranches de littéraux et de valeurs `String` nous amène à une amélioration supplémentaire de `first_word`, qui est sa signature :

```rust
    fn first_word(s: &String) -> &str {
```

Un Rustacien plus expérimenté écrirait plutôt la signature montrée dans l'exemple ci-dessous, car elle nous permet d'utiliser la même fonction sur les valeurs `String` et les valeurs `&str`.

```rust
    fn first_word(s: &str) -> &str {
```

##### Améliorer la fonction first_word en utilisant une tranche de chaîne pour le type du paramètre s

Si nous avons une tranche de chaîne, nous pouvons la passer directement. Si nous avons une `String`, nous pouvons passer une tranche de la `String` entière. Définir une fonction pour prendre une tranche de chaîne au lieu d'une référence à une `String` rend notre API plus générale et utile sans perdre de fonctionnalité :

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word fonctionne sur les tranches de `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word fonctionne sur les tranches de littéraux de chaîne
    let word = first_word(&my_string_literal[..]);

    // Comme les littéraux de chaîne *sont* déjà des tranches de chaîne,
    // cela fonctionne aussi, sans la syntaxe de tranche !
    let word = first_word(my_string_literal);
}
```