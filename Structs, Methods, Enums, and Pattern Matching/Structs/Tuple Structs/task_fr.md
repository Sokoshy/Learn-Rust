### Utiliser des structures tuples sans champs nommés pour créer différents types

Vous pouvez également définir des structures qui ressemblent à des tuples, appelées *structures tuples*. Les structures tuples ont la signification supplémentaire que le nom de la structure fournit mais n'ont pas de noms associés à leurs champs; elles ont simplement les types des champs. Les structures tuples sont utiles lorsque vous souhaitez donner un nom à l'ensemble du tuple et faire en sorte que le tuple soit d'un type différent des autres tuples, et nommer chaque champ comme dans une structure régulière serait verbeux ou redondant.

Pour définir une structure tuple, commencez avec le mot-clé `struct` et le nom de la structure suivi des types dans le tuple. Par exemple, voici des définitions et utilisations de deux structures tuples nommées `Color` et `Point` :

```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```

Notez que les valeurs `black` et `origin` sont de types différents, car ce sont des instances de structures tuples différentes. Chaque structure que vous définissez est son propre type, même si les champs à l'intérieur de la structure ont les mêmes types. Par exemple, une fonction qui prend un paramètre de type `Color` ne peut pas prendre un `Point` comme argument, même si les deux types sont composés de trois valeurs `i32`. À part cela, les instances de structures tuples se comportent comme des tuples : vous pouvez les déconstruire en leurs éléments individuels, vous pouvez utiliser un `.` suivi de l'indice pour accéder à une valeur individuelle, etc.

### Structures unitaires sans aucun champ

Vous pouvez également définir des structures qui n'ont aucun champ ! Celles-ci sont appelées *structures unitaires* parce qu'elles se comportent de manière similaire à `()`, le type unité. Les structures unitaires peuvent être utiles dans des situations où vous devez implémenter un trait sur un type mais que vous n'avez aucunes données que vous souhaitez stocker dans le type lui-même. Nous discuterons des traits [plus tard](course://Generic+Types,+Traits,+and Lifetime/Traits/Traits).