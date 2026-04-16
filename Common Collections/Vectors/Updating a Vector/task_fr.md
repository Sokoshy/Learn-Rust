### Mise à jour d'un vecteur

Pour créer un vecteur et ensuite y ajouter des éléments, nous pouvons utiliser la méthode `push`, comme montré dans l'extrait de code ci-dessous.

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

#### Utiliser la méthode `push` pour ajouter des valeurs à un vecteur

Comme pour toute variable, si nous voulons pouvoir changer sa valeur, nous devons la rendre mutable en utilisant le mot-clé `mut`, comme discuté dans le chapitre "Concepts de programmation courants". Les nombres que nous plaçons à l'intérieur sont tous de type `i32`, et Rust en déduit cela à partir des données, donc nous n'avons pas besoin de l'annotation `Vec<i32>`.