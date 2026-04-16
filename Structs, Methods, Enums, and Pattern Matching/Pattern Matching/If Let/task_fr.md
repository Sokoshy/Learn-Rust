## Contrôle de flux concis avec `if let`

La syntaxe `if let` vous permet de combiner `if` et `let` pour gérer de manière moins verbeuse les valeurs qui correspondent à un motif tout en ignorant le reste. Considérons le programme ci-dessous qui effectue un filtrage sur une valeur `Option<u8>` mais souhaite uniquement exécuter le code si la valeur est 3.

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("trois"),
    _ => (),
}
```

Nous voulons faire quelque chose avec la correspondance `Some(3)` mais ne rien faire avec les autres valeurs `Some<u8>` ou la valeur `None`. Pour satisfaire l'expression `match`, nous devons ajouter `_ => ()` après avoir traité juste une variante, ce qui ajoute beaucoup de code standardisé.

Au lieu de cela, nous pourrions écrire cela de manière plus courte en utilisant `if let`. Le code suivant fonctionne de la même manière que le `match` dans l'extrait de code précédent :

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("trois");
}
```

La syntaxe `if let` prend un motif et une expression séparés par un signe égal. Elle fonctionne de la même manière qu'un `match`, où l'expression est donnée à `match` et le motif constitue son premier bras.

Utiliser `if let` signifie moins de frappe, moins d'indentation et moins de code standardisé. Cependant, vous perdez la vérification exhaustive que `match` impose. Choisir entre `match` et `if let` dépend de ce que vous faites dans votre situation particulière et si gagner en concision est un compromis approprié pour perdre la vérification exhaustive.

En d'autres termes, vous pouvez penser à `if let` comme un sucre syntaxique pour un `match` qui exécute le code lorsque la valeur correspond à un motif, puis ignore toutes les autres valeurs.

Nous pouvons inclure un `else` avec un `if let`. Le bloc de code qui accompagne le `else` est le même que le bloc de code qui irait avec le cas `_` dans l'expression `match` qui est équivalente à `if let` et `else`. Rappelez-vous la définition de l'énumération `Coin` dans la liste de la section précédente, où la variante `Quarter` détenait également une valeur `UsState`. Si nous voulions compter toutes les pièces autres que les quarters que nous voyons tout en annonçant l'état des quarters, nous pourrions le faire avec une expression `match` comme ceci :

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("Quarter d'état de {:?}!", state),
    _ => count += 1,
}
```

Ou nous pourrions utiliser une expression `if let` et `else` comme ceci :

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("Quarter d'état de {:?}!", state);
} else {
    count += 1;
}
```

Si vous avez une situation dans laquelle la logique de votre programme est trop verbeuse pour s'exprimer en utilisant un `match`, souvenez-vous que `if let` est également présent dans votre boîte à outils Rust.