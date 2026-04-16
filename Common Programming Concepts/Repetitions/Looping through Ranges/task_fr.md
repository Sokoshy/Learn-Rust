## Parcourir les plages avec for

La sécurité et la concision des boucles `for` font d'elles la construction de boucle la plus couramment utilisée en Rust. Même dans des situations où vous souhaitez exécuter du code un certain nombre de fois, comme dans l'exemple de compte à rebours qui utilisait une boucle `while` dans "Exemple d'utilisation d'une boucle while pour exécuter du code tant qu'une condition est vraie", la plupart des Rustaciens utiliseraient une boucle `for`. La façon de le faire serait d'utiliser une `Range`, qui est un type fourni par la bibliothèque standard générant tous les nombres en séquence en commençant par un nombre et terminant avant un autre nombre.

Voici à quoi ressemblerait le compte à rebours en utilisant une boucle `for` et une autre méthode dont nous n'avons pas encore parlé, `rev`, pour inverser la plage :

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("DÉCOLLAGE!!!");
}
```

Ce code est un peu plus sympathique, n'est-ce pas ?