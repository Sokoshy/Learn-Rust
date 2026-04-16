### Itérer sur les valeurs dans un vecteur

Si nous voulons accéder à chaque élément d'un vecteur à tour de rôle, nous pouvons itérer sur tous les éléments plutôt que d'utiliser des indices pour accéder à un élément à la fois. Le code ci-dessous montre comment utiliser une boucle `for` pour obtenir des références immuables à chaque élément dans un vecteur de valeurs `i32` et les imprimer.

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
```

#### Imprimer chaque élément d'un vecteur en itérant sur les éléments à l'aide d'une boucle for

Nous pouvons également itérer sur des références mutables à chaque élément dans un vecteur mutable afin de modifier tous les éléments. La boucle `for` dans l'exemple ci-dessous ajoutera `50` à chaque élément.

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

#### Itérer sur les références mutables aux éléments dans un vecteur

Pour changer la valeur à laquelle la référence mutable fait référence, nous devons utiliser l'opérateur de déréférencement (`*`) pour accéder à la valeur de `i` avant de pouvoir utiliser l'opérateur `+=`. Vous pouvez en lire plus sur l'opérateur de déréférencement dans la section “Suivre le pointeur jusqu'à la valeur avec l'opérateur de déréférencement” du Chapitre 15 dans le [livre Rust][book].

[book]: https://doc.rust-lang.org/stable/book/ch15-02-deref.html?highlight=dereference#following-the-pointer-to-the-value