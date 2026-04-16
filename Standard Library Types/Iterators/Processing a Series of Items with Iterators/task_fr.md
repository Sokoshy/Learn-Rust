## Traitement d'une série d'éléments avec des itérateurs

Le motif de l'itérateur vous permet d'effectuer une tâche sur une séquence d'éléments à tour de rôle. Un itérateur est responsable de la logique pour parcourir chaque élément et déterminer quand la séquence est terminée. Lorsque vous utilisez des itérateurs, vous n'avez pas à réimplémenter cette logique vous-même.

En Rust, les itérateurs sont _paresseux_, ce qui signifie qu'ils n'ont aucun effet tant que vous n'appelez pas des méthodes qui consomment l'itérateur pour l'utiliser. Par exemple, le code dans l'extrait ci-dessous crée un itérateur sur les éléments du vecteur `v1` en appelant la méthode `iter` définie sur `Vec<T>`. Ce code en lui-même ne fait rien d'utile.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

##### création d'un itérateur

Une fois que nous avons créé un itérateur, nous pouvons l'utiliser de diverses manières. Dans "Concepts de Programmation Courants/Si", nous avons utilisé des itérateurs avec des boucles `for` pour exécuter du code sur chaque élément, bien que nous ayons éludé ce que faisait l'appel à `iter` jusqu'à maintenant.

L'exemple dans l'extrait de code suivant sépare la création de l'itérateur de l'utilisation de l'itérateur dans la boucle `for`. L'itérateur est stocké dans la variable `v1_iter`, et aucune itération n'a lieu à ce moment-là. Lorsque la boucle `for` est appelée en utilisant l'itérateur dans `v1_iter`, chaque élément de l'itérateur est utilisé lors d'une itération de la boucle, ce qui affiche chaque valeur.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
```

##### utiliser un itérateur dans une boucle for

Dans les langages qui n'ont pas d'itérateurs fournis par leurs bibliothèques standard, vous écririez probablement cette même fonctionnalité en commençant une variable à l'index 0, en utilisant cette variable pour entrer dans le vecteur afin d'obtenir une valeur, et en incrémentant la valeur de la variable dans une boucle jusqu'à ce qu'elle atteigne le nombre total d'éléments dans le vecteur.

Les itérateurs gèrent toute cette logique pour vous, réduisant le code répétitif que vous pourriez potentiellement mal gérer. Les itérateurs vous offrent plus de flexibilité pour utiliser la même logique avec de nombreux types de séquences différents, pas seulement avec des structures de données dans lesquelles vous pouvez entrer, comme les vecteurs. Examinons comment les itérateurs font cela.