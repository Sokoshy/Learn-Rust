### Boucles conditionnelles avec while

Il est souvent utile pour un programme d'évaluer une condition au sein d'une boucle. Tant que la condition est vraie, la boucle s'exécute. Lorsque la condition cesse d'être vraie, le programme appelle `break`, arrêtant la boucle. Ce type de boucle pourrait être implémenté en utilisant une combinaison de `loop`, `if`, `else` et `break`; vous pourriez essayer cela dans un programme si vous le souhaitez.

Cependant, ce schéma est si courant que Rust possède une construction de langage intégrée pour cela, appelée boucle `while`. L'exemple ci-dessous utilise while : le programme boucle trois fois en comptant à rebours à chaque fois, puis, après la boucle, il affiche un autre message et se termine.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("DÉCOLLAGE !!!");
}
```
##### Exemple d'utilisation d'une boucle while pour exécuter du code tant qu'une condition est vraie

Cette construction élimine beaucoup d'imbrications qui seraient nécessaires si vous utilisiez `loop`, `if`, `else` et `break`, et c'est plus clair. Tant qu'une condition est vraie, le code s'exécute ; sinon, il sort de la boucle.