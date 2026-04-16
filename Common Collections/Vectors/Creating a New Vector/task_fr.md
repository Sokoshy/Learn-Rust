### Création d'un nouveau vecteur

Pour créer un vecteur vide, nous pouvons appeler la fonction `Vec::new`, comme illustré dans le code ci-dessous.

```rust
    let v: Vec<i32> = Vec::new();
```

#### Création d'un nouveau vecteur vide pour contenir des valeurs de type i32

Notez que nous avons ajouté une annotation de type ici. Comme nous n'insérons aucune valeur dans ce vecteur, Rust ne sait pas quel type d'éléments nous prévoyons de stocker. C'est un point important. Les vecteurs sont implémentés en utilisant des génériques ; nous verrons comment utiliser les génériques avec vos propres types dans le chapitre "Types génériques, Traits et Durée de vie". Pour l'instant, sachez que le type `Vec<T>` fourni par la bibliothèque standard peut contenir n'importe quel type, et lorsqu'un vecteur spécifique contient un type spécifique, le type est spécifié entre crochets angulaires. Dans le code ci-dessus, nous avons indiqué à Rust que le `Vec<T>` dans `v` contiendra des éléments de type `i32`.

Dans un code plus réaliste, Rust peut souvent déduire le type de valeur que vous souhaitez stocker une fois que vous insérez des valeurs, vous avez donc rarement besoin de faire cette annotation de type. Il est plus courant de créer un `Vec<T>` qui a des valeurs initiales, et Rust fournit la macro `vec!` pour plus de commodité. La macro créera un nouveau vecteur qui contiendra les valeurs que vous lui fournissez. La liste ci-dessous crée un nouveau `Vec<i32>` qui contient les valeurs `1`, `2` et `3`. Le type entier est `i32` car c'est le type entier par défaut, comme nous l'avons discuté dans la section “Types de données” du chapitre "Concepts de programmation communs".

```rust
    let v = vec![1, 2, 3];
```

#### Création d'un nouveau vecteur contenant des valeurs

Parce que nous avons donné des valeurs initiales `i32`, Rust peut déduire que le type de `v` est `Vec<i32>`, et l'annotation de type n'est pas nécessaire. Ensuite, nous verrons comment modifier un vecteur.