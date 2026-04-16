### Enums avec des données et des méthodes

Examinons un autre exemple d'un enum dans la liste ci-dessous : celui-ci a une grande variété de types intégrés dans ses variantes.

```rust
enum Message {
    Quitter,
    Déplacer { x: i32, y: i32 },
    Écrire(String),
    ChangerCouleur(i32, i32, i32),
}
```

#### Un enum `Message` dont les variantes stockent chacune des quantités et types de valeurs différents

Cet enum a quatre variantes avec des types différents :

* `Quitter` n'a aucune donnée associée.
* `Déplacer` inclut une structure anonyme à l'intérieur.
* `Écrire` inclut une seule `String`.
* `ChangerCouleur` inclut trois valeurs `i32`.

Définir un enum avec des variantes comme celles de la liste ci-dessus est similaire à définir différents types de définitions de structure, sauf que l'enum n'utilise pas le mot-clé `struct` et que toutes les variantes sont regroupées sous le type `Message`. Les structures suivantes pourraient contenir les mêmes données que les variantes d'enum précédentes :

```rust
struct QuitterMessage; // structure unité
struct DéplacerMessage {
    x: i32,
    y: i32,
}
struct ÉcrireMessage(String); // structure tuple
struct ChangerCouleurMessage(i32, i32, i32); // structure tuple
```

Mais si nous utilisions les différentes structures, qui ont chacune leur propre type, nous ne pourrions pas définir aussi facilement une fonction pour prendre n'importe lequel de ces types de messages comme nous pourrions le faire avec l'enum `Message` défini dans l'extrait de code précédent, qui est un type unique.

Il y a une autre similarité entre les enums et les structures : tout comme nous pouvons définir des méthodes sur les structures en utilisant `impl`, nous pouvons également définir des méthodes sur les enums. Voici une méthode nommée `appel` que nous pourrions définir sur notre enum `Message` :

```rust
impl Message {
    fn appel(&self) {
        // le corps de la méthode serait défini ici
    }
}

let m = Message::Écrire(String::from("bonjour"));
m.appel();
```

Le corps de la méthode utiliserait `self` pour obtenir la valeur sur laquelle nous avons appelé la méthode. Dans cet exemple, nous avons créé une variable `m` qui a la valeur `Message::Écrire(String::from("bonjour"))`, et c'est ce que `self` sera dans le corps de la méthode `appel` lorsque `m.appel()` s'exécutera.

Examinons un autre enum dans la bibliothèque standard qui est très courant et utile : `Option`.