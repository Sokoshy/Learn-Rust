### Mise à jour d'une String

Une `String` peut s'agrandir et son contenu peut changer, tout comme le contenu d'un `Vec<T>`, si vous y ajoutez plus de données. De plus, vous pouvez utiliser facilement l'opérateur `+` ou la macro `format!` pour concaténer des valeurs `String`.

#### Ajouter à une String avec `push_str` et `push`

Nous pouvons agrandir une `String` en utilisant la méthode `push_str` pour ajouter une tranche de chaîne, comme indiqué dans l'extrait de code ci-dessous.

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```

##### Exemple d'ajout d'une tranche de chaîne à une String en utilisant la méthode push_str

Après ces deux lignes, `s` contiendra `foobar`. La méthode `push_str` prend une tranche de chaîne car nous ne voulons pas nécessairement prendre possession du paramètre. Par exemple, le prochain extrait de code montre qu'il serait dommage si nous ne pouvions pas utiliser `s2` après avoir ajouté son contenu à `s1`.

```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 est {}", s2);
```

##### Utiliser une tranche de chaîne après avoir ajouté son contenu à une String

Si la méthode `push_str` prenait possession de `s2`, nous ne pourrions pas imprimer sa valeur à la dernière ligne. Cependant, ce code fonctionne comme prévu !

La méthode `push` prend un seul caractère comme paramètre et l'ajoute à la `String`. L'extrait de code ci-dessous montre du code qui ajoute la lettre _l_ à une `String` en utilisant la méthode `push`.

```rust
    let mut s = String::from("lo");
    s.push('l');
```

##### Ajouter un caractère à une valeur String en utilisant push

En conséquence de ce code, `s` contiendra `lol`.

#### Concaténation avec l'opérateur `+` ou la macro format!

Souvent, vous voudrez combiner deux chaînes existantes. Une façon de le faire est d'utiliser l'opérateur `+`, comme indiqué ci-dessous.

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // notez que s1 a été déplacée ici et ne peut plus être utilisée
```

##### Utiliser l'opérateur + pour combiner deux valeurs String en une nouvelle valeur String

La chaîne `s3` contiendra `Hello, world!` en conséquence de ce code. La raison pour laquelle `s1` n'est plus valide après l'addition, et la raison pour laquelle nous avons utilisé une référence à `s2`, est liée à la signature de la méthode appelée lorsque nous utilisons l'opérateur `+`. L'opérateur `+` utilise la méthode `add`, dont la signature ressemble à ceci :

```rust
    fn add(self, s: &str) -> String {
```

Ce n'est pas la signature exacte qui est dans la bibliothèque standard : dans la bibliothèque standard, `add` est définie en utilisant des génériques. Ici, nous regardons la signature de `add` avec des types concrets substitués aux génériques, ce qui se produit lorsque nous appelons cette méthode avec des valeurs `String`. Nous aborderons les génériques dans le chapitre "Types génériques, Traits et Durées de vie". Cette signature nous donne les indices nécessaires pour comprendre les subtilités de l'opérateur `+`.

Premièrement, `s2` a un `&`, ce qui signifie que nous ajoutons une _référence_ de la seconde chaîne à la première chaîne en raison du paramètre `s` dans la fonction `add` : nous ne pouvons ajouter qu'une `&str` à une `String` ; nous ne pouvons pas additionner deux valeurs `String`. Mais attendez - le type de `&s2` est `&String`, pas `&str`, comme spécifié dans le second paramètre de `add`. Alors pourquoi la liste "Utiliser l'opérateur + pour combiner deux valeurs String en une nouvelle valeur String" se compile-t-elle ?

La raison pour laquelle nous pouvons utiliser `&s2` dans l'appel à `add` est que le compilateur peut _coercer_ l'argument `&String` en un `&str`. Lorsque nous appelons la méthode `add`, Rust utilise une _coercition de déréférencement_, qui ici transforme `&s2` en `&s2[..]`. Nous aborderons la coercition de déréférencement plus en profondeur dans le Chapitre 15 du [Rust Book](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html). Parce que `add` ne prend pas possession du paramètre `s`, `s2` restera une `String` valide après cette opération.

Deuxièmement, nous voyons dans la signature que `add` prend possession de `self`, car `self` n'a pas de `&`. Cela signifie que `s1` dans la liste "Utiliser l'opérateur + pour combiner deux valeurs String en une nouvelle valeur String" sera déplacée dans l'appel `add` et ne sera plus valide par la suite. Donc, bien que `let s3 = s1 + &s2;` semble copier les deux chaînes et en créer une nouvelle, cette instruction prend en fait possession de `s1`, ajoute une copie du contenu de `s2`, et retourne ensuite la possession du résultat. Autrement dit, il semble faire beaucoup de copies, mais ne le fait pas ; l'implémentation est plus efficace que de copier.

Si nous avons besoin de concaténer plusieurs chaînes, le comportement de l'opérateur `+` devient peu pratique :

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

À ce point, `s` sera `tic-tac-toe`. Avec tous ces caractères `+` et `"`, il est difficile de voir ce qui se passe. Pour une combinaison de chaînes plus complexe, nous pouvons utiliser la macro `format!` :

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
```

Ce code attribue également à `s` la valeur `tic-tac-toe`. La macro `format!` fonctionne de la même manière que `println!`, mais au lieu d'afficher la sortie à l'écran, elle retourne une `String` avec le contenu. La version du code utilisant `format!` est bien plus lisible et ne prend pas possession de ses paramètres.