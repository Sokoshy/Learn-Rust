### Modèles qui se lient à des valeurs

Une autre fonctionnalité utile des bras de match est qu'ils peuvent se lier aux parties des valeurs qui correspondent au modèle. C'est ainsi que nous pouvons extraire des valeurs des variantes d'énumération.

À titre d'exemple, modifions une de nos variantes d'énumération pour contenir des données à l'intérieur. De 1999 à 2008, les États-Unis ont frappé des quarts de dollar avec des designs différents pour chacun des 50 États sur une face. Aucune autre pièce n’a reçu de designs d’État, donc seuls les quarts de dollar ont cette valeur supplémentaire. Nous pouvons ajouter cette information à notre `enum` en changeant la variante `Quarter` pour inclure une valeur `UsState` stockée à l'intérieur, ce que nous avons fait ici dans le code ci-dessous.

```rust
#[derive(Debug)] // pour que nous puissions inspecter l'état dans un instant
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

#### Un `enum Coin` dans lequel la variante `Quarter` contient également une valeur `UsState`

Imaginons qu’un de nos amis essaie de collectionner les 50 quarts de dollar d'État. Pendant que nous trions notre petite monnaie par type de pièce, nous mentionnerons également le nom de l'État associé à chaque quart de dollar, pour que s’il s’agit d’un qu’il n’a pas, il puisse l’ajouter à sa collection.

Dans l'expression match pour ce code, nous ajoutons une variable appelée `state` au modèle qui correspond aux valeurs de la variante `Coin::Quarter`. Lorsqu'un `Coin::Quarter` correspond, la variable `state` se liera à la valeur de l'état de ce quart de dollar. Ensuite, nous pouvons utiliser `state` dans le code pour ce bras, comme suit :

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quart de dollar de l'État de {:?}!", state);
            25
        }
    }
}
```

Si nous devions appeler `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` serait `Coin::Quarter(UsState::Alaska)`. Lorsque nous comparons cette valeur avec chacun des bras de match, aucun d'eux ne correspond jusqu'à ce que nous atteignions `Coin::Quarter(state)`. À ce moment-là, le lien pour `state` sera la valeur `UsState::Alaska`. Nous pouvons alors utiliser ce lien dans l'expression `println!`, obtenant ainsi la valeur d'état interne de la variante d'énumération `Coin` pour `Quarter`.