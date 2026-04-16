### Dans les définitions d'énumérations

Comme nous l'avons fait avec les structures, nous pouvons définir des énumérations pour contenir des types de données génériques dans leurs variantes. Reconsidérons l'énumération `Option<T>` que la bibliothèque standard fournit, que nous avons utilisée dans le chapitre "Énumérations" :

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Cette définition devrait maintenant être plus claire pour vous. Comme vous pouvez le voir, `Option<T>` est une énumération générique sur le type `T` et a deux variantes : `Some`, qui contient une valeur de type `T`, et une variante `None` qui ne contient aucune valeur. En utilisant l'énumération `Option<T>`, nous pouvons exprimer le concept abstrait d'avoir une valeur optionnelle, et parce que `Option<T>` est générique, nous pouvons utiliser cette abstraction quel que soit le type de la valeur optionnelle.

Les énumérations peuvent aussi utiliser plusieurs types génériques. La définition de l'énumération `Result` que nous avons utilisée dans le chapitre "Erreurs récupérables et non récupérables" en est un exemple :

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

L'énumération `Result` est générique sur deux types, `T` et `E`, et a deux variantes : `Ok`, qui contient une valeur de type `T`, et `Err`, qui contient une valeur de type `E`. Cette définition rend pratique l'utilisation de l'énumération `Result` partout où nous avons une opération qui pourrait réussir (retourner une valeur de type `T`) ou échouer (retourner une erreur de type `E`). En fait, c'est ce que nous avons utilisé pour ouvrir un fichier dans l'extrait de code "Ouverture d'un fichier" (section "Erreurs récupérables avec Result" dans "Gestion des erreurs"), où `T` était rempli avec le type `std::fs::File` lorsque le fichier était ouvert avec succès et `E` était rempli avec le type `std::io::Error` lorsqu'il y avait des problèmes lors de l'ouverture du fichier.

Lorsque vous identifiez des situations dans votre code avec plusieurs définitions de structures ou d'énumérations qui ne diffèrent que par les types des valeurs qu'elles contiennent, vous pouvez éviter les duplications en utilisant des types génériques à la place.