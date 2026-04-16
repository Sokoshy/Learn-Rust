## Conversions de type

Rust offre de multiples façons de convertir une valeur d'un type donné en un autre type.

La forme la plus simple de conversion de type est une expression de transtypage (type cast). Elle est désignée par l'opérateur binaire `as`. Par exemple, `println!("{}", 1 + 1.0);` ne se compilerait pas, car `1` est un entier alors que `1.0` est un flottant. Cependant, `println!("{}", 1 as f32 + 1.0)` devrait se compiler. L'exercice [`using_as`](using_as.rs) tente de couvrir cela.

Rust propose également des traits qui facilitent les conversions de type lors de leur implémentation. Ces traits se trouvent dans le module [`convert`](https://doc.rust-lang.org/std/convert/index.html).
Les traits sont les suivants :
- `From` et `Into` couverts dans [`from_into`](from_into.rs)
- `TryFrom` et `TryInto` couverts dans [`try_from_into`](try_from_into.rs)
- `AsRef` et `AsMut` couverts dans [`as_ref_mut`](as_ref_mut.rs)

De plus, le module `std::str` offre un trait appelé [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) qui aide à convertir des chaînes de caractères en types cibles via la méthode `parse` sur des chaînes. S'il est correctement implémenté pour un type donné `Person`, alors `let p: Person = "Mark,20".parse().unwrap()` devrait se compiler et s'exécuter sans panique.

Ceux-ci devraient être les principaux moyens ***dans la bibliothèque standard*** pour convertir des données dans les types souhaités.

#### sections du livre Rust

Ceux-ci ne sont pas directement couverts dans le livre, mais la bibliothèque standard dispose d'une excellente documentation sur [les conversions ici](https://doc.rust-lang.org/std/convert/index.html). Le trait `FromStr` est également couvert [ici](https://doc.rust-lang.org/std/str/trait.FromStr.html).