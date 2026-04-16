## Autres types de tranches

Les tranches de chaînes, comme vous pouvez l'imaginer, sont spécifiques aux chaînes de caractères. Mais il existe aussi un type de tranche plus général. Considérons ce tableau :

```rust
    let a = [1, 2, 3, 4, 5];
```

De la même manière que nous pourrions vouloir faire référence à une partie d'une chaîne, nous pourrions vouloir faire référence à une partie d'un tableau. Nous le ferions ainsi :

```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
```

Cette tranche a le type `&[i32]`. Elle fonctionne de la même manière que les tranches de chaînes, en stockant une référence au premier élément et une longueur. Vous utiliserez ce type de tranche pour toutes sortes d'autres collections. Nous discuterons de ces collections en détail [plus tard](course://Common Collections/Vectors/Intro).