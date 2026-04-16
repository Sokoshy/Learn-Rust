### Créer une nouvelle table de hachage

Vous pouvez créer une table de hachage vide avec `new` et ajouter des éléments avec `insert`. Dans le code ci-dessous, nous suivons les scores de deux équipes dont les noms sont Bleu et Jaune. L'équipe Bleue commence avec 10 points, et l'équipe Jaune commence avec 50.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

#### Créer une nouvelle table de hachage et insérer des clés et des valeurs

Notez que nous devons d'abord `use` le `HashMap` de la partie des collections de la bibliothèque standard. Parmi nos trois collections courantes, celle-ci est la moins souvent utilisée, donc elle n'est pas incluse dans les fonctionnalités importées automatiquement dans le préambule. Les tables de hachage ont également moins de support de la bibliothèque standard ; il n'y a pas de macro intégrée pour les construire, par exemple.

Tout comme les vecteurs, les tables de hachage stockent leurs données sur le tas. Ce `HashMap` a des clés de type `String` et des valeurs de type `i32`. Comme les vecteurs, les tables de hachage sont homogènes : toutes les clés doivent avoir le même type et toutes les valeurs doivent avoir le même type.

Une autre façon de construire une table de hachage est d'utiliser des itérateurs et la méthode `collect` sur un vecteur de tuples, où chaque tuple se compose d'une clé et de sa valeur. Nous entrerons plus en détail sur les itérateurs et leurs méthodes associées dans la section "Itérateurs" du chapitre "Itérateurs et Closures". La méthode `collect` rassemble les données dans un certain nombre de types de collections, y compris `HashMap`. Par exemple, si nous avions les noms des équipes et les scores initiaux dans deux vecteurs séparés, nous pourrions utiliser la méthode `zip` pour créer un vecteur de tuples où "Blue" est associé à 10, et ainsi de suite. Ensuite, nous pourrions utiliser la méthode `collect` pour transformer ce vecteur de tuples en une table de hachage, comme montré dans l'exemple ci-dessous.

```rust
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
```

#### Créer une table de hachage à partir d'une liste d'équipes et d'une liste de scores

L'annotation de type `HashMap<_, _>` est nécessaire ici parce qu'il est possible de `collect` dans de nombreuses structures de données différentes et Rust ne sait pas laquelle vous voulez, à moins de spécifier. Pour les paramètres des types de clé et de valeur, cependant, nous utilisons des tirets bas, et Rust peut déduire les types que contient la table de hachage en fonction des types des données dans les vecteurs. Dans le code ci-dessus, le type de clé sera `String` et le type de valeur sera `i32`, tout comme les types étaient dans le premier exemple de cette section.