### Méthodes qui produisent d'autres itérateurs

D'autres méthodes définies sur le trait `Iterator`, connues sous le nom de _modificateurs d'itérateur_, vous permettent de transformer des itérateurs en différents types d'itérateurs. Vous pouvez enchaîner plusieurs appels à des modificateurs d'itérateur pour effectuer des actions complexes de manière lisible. Mais comme tous les itérateurs sont paresseux, vous devez appeler l'une des méthodes de consommateur pour obtenir des résultats des appels aux modificateurs d'itérateur.

Le fragment de code suivant montre un exemple d'appel à la méthode modificateur d'itérateur `map`, qui prend une fermeture pour l'appeler sur chaque élément afin de produire un nouvel itérateur. La fermeture ici crée un nouvel itérateur dans lequel chaque élément du vecteur a été incrémenté de 1. Cependant, ce code produit un avertissement :

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```

##### Appeler le modificateur d'itérateur map pour créer un nouvel itérateur

L'avertissement que nous recevons est le suivant :

```text
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed
```

Le code dans le dernier exemple ne fait rien ; la fermeture que nous avons spécifiée n'est jamais appelée. L'avertissement nous rappelle pourquoi : les modificateurs d'itérateur sont paresseux, et nous devons consommer l'itérateur ici.

Pour corriger cela et consommer l'itérateur, nous allons utiliser la méthode `collect`, qui est discutée dans le **[Chapitre 12](https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html)** du Rust Book, avec `env::args`. Cette méthode consomme l'itérateur et collecte les valeurs résultantes dans un type de données de collection.

Dans l'exemple suivant, nous collectons les résultats de l'itération sur l'itérateur qui est retourné par l'appel à `map` dans un vecteur. Ce vecteur contiendra finalement chaque élément du vecteur original incrémenté de 1.

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

##### Appeler la méthode map pour créer un nouvel itérateur puis appeler la méthode collect pour consommer le nouvel itérateur et créer un vecteur

Comme `map` prend une fermeture, nous pouvons spécifier n'importe quelle opération que nous voulons effectuer sur chaque élément. C'est un excellent exemple de la façon dont les fermetures vous permettent de personnaliser certains comportements tout en réutilisant le comportement d'itération que fournit le trait `Iterator`.