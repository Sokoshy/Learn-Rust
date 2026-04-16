## Types de données

Chaque valeur en Rust appartient à un certain _type de données_, qui indique à Rust quel type de données est spécifié afin qu'il sache comment travailler avec ces données. Dans cette leçon, nous examinerons les types scalaires, des types qui représentent une valeur unique.

Gardez à l'esprit que Rust est un _langage à typage statique_, ce qui signifie qu'il doit connaître les types de toutes les variables au moment de la compilation. Le compilateur peut généralement déduire quel type nous voulons utiliser en se basant sur la valeur et comment nous l'utilisons. Dans les cas où plusieurs types sont possibles, comme lorsqu'on convertit une `String` en un type numérique en utilisant `parse`, nous devons ajouter une annotation de type, comme ceci :

```rust
let guess: u32 = "42".parse().expect("Pas un nombre !");
```

Si nous n'ajoutons pas l'annotation de type ici, Rust affichera l'erreur suivante, ce qui signifie que le compilateur a besoin de plus d'informations de notre part pour savoir quel type nous voulons utiliser :

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Pas un nombre !");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

Vous verrez des annotations de type différentes pour d'autres types de données.

### Types scalaires

Un type _scalaire_ représente une valeur unique. Rust possède quatre principaux types scalaires : les entiers, les nombres à virgule flottante, les booléens et les caractères. Vous pourriez reconnaître ces types d'autres langages de programmation. Voyons comment ils fonctionnent en Rust.