### Méthodes pour itérer sur des chaînes de caractères

Heureusement, vous pouvez accéder aux éléments d'une chaîne de différentes manières.

Si vous avez besoin d'effectuer des opérations sur des valeurs scalaires Unicode individuelles, la meilleure façon de le faire est d'utiliser la méthode `chars`. Appeler `chars` sur “नमस्ते” sépare et retourne six valeurs de type `char`, et vous pouvez itérer sur le résultat pour accéder à chaque élément :

```rust
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
```

Ce code imprimera ce qui suit :

```text
    न
    म
    स
    ्
    त
    े
```

La méthode `bytes` retourne chaque octet brut, ce qui peut être approprié pour votre domaine :

```rust
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
```

Ce code imprimera les 18 octets qui composent cette `String` :

```text
    224
    164
    // --snip--
    165
    135
```

Mais veillez à vous rappeler que des valeurs scalaires Unicode valides peuvent être composées de plus d'un octet.

Obtenir des clusters de graphèmes à partir des chaînes est complexe, donc cette fonctionnalité n'est pas fournie par la bibliothèque standard. Des crates sont disponibles sur [crates.io](https://crates.io) si c'est la fonctionnalité dont vous avez besoin.