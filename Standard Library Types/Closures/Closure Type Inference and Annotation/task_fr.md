### Inférence de type et annotation des fermetures

Il existe d'autres différences entre les fonctions et les fermetures. Les fermetures ne nécessitent généralement pas d'annoter les types des paramètres ou la valeur de retour comme le font les fonctions `fn`. Les annotations de type sont requises pour les fonctions car elles font partie d'une interface explicite exposée à vos utilisateurs. Définir cette interface de manière stricte est important pour s'assurer que tout le monde s'accorde sur les types de valeurs qu'une fonction utilise et renvoie. Mais les fermetures ne sont pas utilisées dans une interface exposée comme celle-ci : elles sont stockées dans des variables et utilisées sans les nommer ni les exposer aux utilisateurs de notre bibliothèque.

Les fermetures sont généralement courtes et pertinentes uniquement dans un contexte restreint plutôt que dans un scénario quelconque. Dans ces contextes limités, le compilateur peut inférer les types des paramètres et le type de retour, de la même manière qu'il peut inférer les types de la plupart des variables (il existe de rares cas où le compilateur nécessite également des annotations de type pour les fermetures).

Comme pour les variables, nous pouvons ajouter des annotations de type si nous voulons accroître l'explicitation et la clarté au prix d'être plus verbeux que nécessaire. Annoter les types pour une fermeture ressemblerait à la définition montrée ci-dessous :

```rust
    let expensive_closure = |num: u32| -> u32 {
        println!("calcul en cours...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

##### Exemple d'ajout d'annotations de type optionnelles des types de paramètre et de valeur de retour dans la fermeture

Avec les annotations de type ajoutées, la syntaxe des fermetures ressemble davantage à la syntaxe des fonctions. Voici une comparaison verticale de la syntaxe pour la définition d'une fonction qui ajoute 1 à son paramètre et d'une fermeture ayant le même comportement. Nous avons ajouté des espaces pour aligner les parties pertinentes. Cela illustre comment la syntaxe des fermetures est similaire à celle des fonctions à l'exception de l'utilisation de barres et de la quantité de syntaxe qui est facultative:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

La première ligne montre une définition de fonction, et la deuxième ligne montre une définition de fermeture entièrement annotée. La troisième ligne retire les annotations de type de la définition de la fermeture, et la quatrième ligne retire les accolades, qui sont facultatives parce que le corps de la fermeture ne contient qu'une seule expression. Ce sont toutes des définitions valides qui produiront le même comportement lorsqu'elles seront appelées. Appeler les fermetures est nécessaire pour que `add_one_v3` et `add_one_v4` puissent être compilés car les types seront déduits de leur utilisation.

Les définitions de fermetures auront un type concret déduit pour chacun de leurs paramètres et pour leur valeur de retour. Par exemple, le code suivant montre la définition d'une courte fermeture qui renvoie simplement la valeur qu'elle reçoit en paramètre. Cette fermeture n'est pas très utile sauf aux fins de cet exemple. Notez que nous n'avons ajouté aucune annotation de type à la définition : si nous essayons ensuite d'appeler la fermeture deux fois, en utilisant une `String` comme argument la première fois et un `u32` la deuxième fois, nous obtiendrons une erreur.

```rust
let example_closure = |x| x;

let s = example_closure(String::from("bonjour"));
let n = example_closure(5);
```

##### Exemple de tentative d'appel d'une fermeture dont les types sont inférés avec deux types différents

Le compilateur nous donne cette erreur :

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^- aide : essayez d'utiliser une méthode de conversion : `.to_string()`
  |                             |
  |                             type struct `String` attendu, entier trouvé

Pour plus d'informations sur cette erreur, essayez `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

La première fois que nous appelons `example_closure` avec la valeur `String`, le compilateur infère le type de `x` et le type de retour de la fermeture pour être `String`. Ces types sont ensuite verrouillés dans la fermeture de `example_closure`, et nous obtenons une erreur de type si nous essayons d'utiliser un type différent avec la même fermeture.