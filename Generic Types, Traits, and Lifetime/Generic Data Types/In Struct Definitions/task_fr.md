### Dans les définitions de struct

Nous pouvons également définir des structures pour utiliser un paramètre de type générique dans un ou plusieurs champs en utilisant la syntaxe `<>`. Le code ci-dessous montre comment définir une struct `Point<T>` pour contenir les valeurs de coordonnées `x` et `y` de n'importe quel type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

#### Une struct `Point<T>` qui contient des valeurs `x` et `y` de type `T`

La syntaxe pour utiliser des génériques dans les définitions de struct est similaire à celle utilisée dans les définitions de fonctions. Tout d'abord, nous déclarons le nom du paramètre de type entre crochets angulaires juste après le nom de la struct. Ensuite, nous pouvons utiliser le type générique dans la définition de la struct là où nous spécifierions autrement des types de données concrets.

Notez que, parce que nous avons utilisé un seul type générique pour définir `Point<T>`, cette définition indique que la struct `Point<T>` est générique sur un type `T`, et les champs `x` et `y` sont *tous deux* de ce même type, quel qu'il soit. Si nous créons une instance de `Point<T>` qui a des valeurs de types différents, comme cela a été fait ci-dessous, notre code ne se compilera pas.

```rust,ignore,does_not_compile
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

#### Les champs `x` et `y` doivent être du même type car tous deux ont le même type de donnée générique `T`.

Dans cet exemple, lorsque nous affectons la valeur entière 5 à `x`, nous informons le compilateur que le type générique `T` sera un entier pour cette instance de `Point<T>`. Ensuite, lorsque nous spécifions 4.0 pour `y`, que nous avons défini pour avoir le même type que `x`, nous obtiendrons une erreur de différence de type comme celle-ci :

```console
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ attendu un entier, trouvé un nombre à virgule flottante
```

Pour définir une struct `Point` où `x` et `y` sont tous deux génériques mais pourraient avoir des types différents, nous pouvons utiliser plusieurs paramètres de type génériques. Par exemple, dans l'exemple ci-dessous, nous pouvons changer la définition de `Point` pour être générique sur les types `T` et `U` où `x` est de type `T` et `y` est de type `U`.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

#### Une struct `Point<T, U>` générique sur deux types afin que `x` et `y` puissent être des valeurs de types différents

Maintenant, toutes les instances de `Point` présentées sont autorisées ! Vous pouvez utiliser autant de paramètres de type génériques que vous le souhaitez dans une définition, mais en utiliser plus que quelques-uns rend votre code difficile à lire. Lorsque vous avez besoin de nombreux types génériques dans votre code, cela pourrait indiquer que votre code nécessite une restructuration en morceaux plus petits.