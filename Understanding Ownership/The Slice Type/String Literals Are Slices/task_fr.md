## Les littéraux de chaîne sont des tranches

Rappelons que nous avons parlé des littéraux de chaîne étant stockés dans le binaire. Maintenant que nous connaissons les tranches, nous pouvons comprendre correctement les littéraux de chaîne :

```rust
    let s = "Hello, world!";
```

Le type de `s` ici est `&str` : c’est une tranche pointant vers ce point spécifique du binaire. C’est aussi pourquoi les littéraux de chaîne sont immuables ; `&str` est une référence immuable.