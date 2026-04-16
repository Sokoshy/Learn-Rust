### Hash maps et possession

Pour les types qui implémentent le trait `Copy`, comme `i32`, les valeurs sont copiées dans la hash map. Pour les valeurs possédées comme `String`, les valeurs seront déplacées et la hash map deviendra le propriétaire de ces valeurs, comme démontré ci-dessous.

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name et field_value ne sont plus valides à ce stade, essayez de les utiliser et
    // voyez quelle erreur de compilation vous obtenez !
```

#### Montrer que les clés et les valeurs appartiennent à la hash map une fois qu'elles sont insérées

Nous ne pouvons pas utiliser les variables `field_name` et `field_value` après qu'elles ont été déplacées dans la hash map avec l'appel à `insert`.

Si nous insérons des références aux valeurs dans la hash map, les valeurs ne seront pas déplacées dans la hash map. Les valeurs auxquelles les références pointent doivent être valides au moins aussi longtemps que la hash map est valide. Vous pouvez en savoir plus sur ces questions dans la section [“Validation des références avec les durées de vie”][validating-references-with-lifetimes] du chapitre 10 du Rust Book.

[validating-references-with-lifetimes]:
https://github.com/rust-lang/book/blob/master/src/ch10-03-lifetime-syntax.md