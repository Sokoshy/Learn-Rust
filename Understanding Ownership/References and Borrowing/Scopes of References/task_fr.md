## Portées des références

Notez que la portée d'une référence commence là où elle est introduite et continue jusqu'à la dernière utilisation de cette référence. Par exemple, ce code se compilera parce que la dernière utilisation des références immuables se produit avant l'introduction de la référence mutable :

```rust
    let mut s = String::from("hello");

    let r1 = &s; // pas de problème
    let r2 = &s; // pas de problème
    println!("{} et {}", r1, r2);
    // r1 et r2 ne sont plus utilisés après ce point

    let r3 = &mut s; // pas de problème
    println!("{}", r3);
```

Les portées des références immuables `r1` et `r2` se terminent après le `println!` où elles sont utilisées pour la dernière fois, c'est-à-dire avant que la référence mutable `r3` ne soit créée. Ces portées ne se chevauchent pas, donc ce code est autorisé. La capacité du compilateur à déterminer qu'une référence n'est plus utilisée à un moment avant la fin de la portée s'appelle les durées de vie non lexicales (NLL pour Non-Lexical Lifetimes), et vous pouvez en lire davantage à ce sujet dans [The Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html).

Même si les erreurs d'emprunt peuvent être frustrantes à certains moments, souvenez-vous que c’est le compilateur Rust qui vous indique un bug potentiel tôt (à la compilation plutôt qu'à l'exécution) et vous montre exactement où se trouve le problème. Ainsi, vous n'avez pas besoin de rechercher pourquoi vos données ne sont pas ce que vous pensiez qu’elles étaient.