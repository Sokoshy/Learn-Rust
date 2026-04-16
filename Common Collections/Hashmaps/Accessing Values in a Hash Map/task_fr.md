### Accéder aux valeurs dans une table de hachage

Nous pouvons obtenir une valeur de la table de hachage en fournissant sa clé à la méthode `get`, comme indiqué ci-dessous.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

#### Accéder au score de l'équipe Blue stocké dans la table de hachage

Ici, `score` aura la valeur associée à l'équipe Blue, et le résultat sera `Some(&10)`. Le résultat est enveloppé dans `Some` parce que `get` renvoie une `Option<&V>` ; s'il n'y a pas de valeur pour cette clé dans la table de hachage, `get` renverra `None`. Le programme devra traiter l'`Option` de l'une des manières que nous avons vues dans le chapitre "Énumérations".

Nous pouvons itérer sur chaque paire clé/valeur dans une table de hachage de manière similaire à celle des vecteurs, en utilisant une boucle `for` :

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

Ce code imprimera chaque paire dans un ordre arbitraire :

```text
Yellow: 50
Blue: 10
```