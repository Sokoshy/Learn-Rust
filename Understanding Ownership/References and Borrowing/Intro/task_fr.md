## Introduction

Prendre possession et ensuite rendre la possession avec chaque fonction est un peu fastidieux. Que faire si nous voulons qu'une fonction utilise une valeur sans en prendre la possession ? C'est assez agaçant que tout ce que nous passons en paramètre doit également être renvoyé si nous voulons l'utiliser à nouveau, en plus des données résultantes du corps de la fonction que nous pourrions également vouloir retourner.

Il est possible de retourner plusieurs valeurs en utilisant un tuple, comme montré dans l'extrait ci-dessous.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("La longueur de '{}' est {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() renvoie la longueur d'une String

    (s, length)
}
```

##### Retourner la possession des paramètres

Mais cela est trop cérémonieux et demande beaucoup de travail pour un concept qui devrait être courant. Heureusement pour nous, Rust a une fonctionnalité pour ce concept, appelée _références_.