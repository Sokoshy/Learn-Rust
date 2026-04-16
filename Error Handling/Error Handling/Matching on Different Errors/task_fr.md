### Gestion des erreurs différentes

Le code dans l'exemple suivant effectuera un `panic!` peu importe pourquoi `File::open` a échoué. Ce que nous voulons faire à la place, c'est prendre différentes actions pour différentes raisons d'échec : si `File::open` a échoué parce que le fichier n'existe pas, nous voulons créer le fichier et retourner le handle vers le nouveau fichier. Si `File::open` échoue pour une autre raison — par exemple, parce que nous n'avons pas la permission d'ouvrir le fichier — nous voulons toujours que le code fasse un `panic!` de la même manière que dans l'extrait précédent. Regardez le code suivant, qui ajoute une expression `match` interne.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problème lors de la création du fichier : {:?}", e),
            },
            other_error => {
                panic!("Problème lors de l'ouverture du fichier : {:?}", other_error)
            }
        },
    };
}
```

##### gérer différents types d'erreurs de différentes manières

Le type de valeur que `File::open` retourne à l'intérieur de la variante `Err` est `io::Error`, qui est une structure fournie par la bibliothèque standard. Cette structure a une méthode `kind` que nous pouvons appeler pour obtenir une valeur `io::ErrorKind`. L'énumération `io::ErrorKind` est fournie par la bibliothèque standard et a des variantes représentant les différents types d'erreurs qui pourraient résulter d'une opération `io`. La variante que nous voulons utiliser est `ErrorKind::NotFound`, qui indique que le fichier que nous essayons d'ouvrir n'existe pas encore. Ainsi, nous faisons un `match` sur `f`, mais nous avons également un `match` intérieur sur `error.kind()`.

La condition que nous voulons vérifier dans le `match` intérieur est si la valeur retournée par `error.kind()` est la variante `NotFound` de l'énumération `ErrorKind`. Si c'est le cas, nous essayons de créer le fichier avec `File::create`. Cependant, comme `File::create` pourrait également échouer, nous avons besoin d'un second bras dans l'expression `match` intérieure. Lorsque le fichier ne peut pas être créé, un message d'erreur différent est affiché. Le second bras du `match` extérieur reste le même, donc le programme fait un panic pour toute erreur autre que l'erreur de fichier manquant.

Cela fait beaucoup de `match` ! L'expression `match` est très utile mais aussi très primitive. Dans ["Types de bibliothèque standard/Closures"](course://Standard Library Types/Closures), vous apprendrez à propos des closures ; le type `Result<T, E>` a de nombreuses méthodes qui acceptent une closure et sont implémentées en utilisant des expressions `match`. Utiliser ces méthodes rendra votre code plus concis. Un Rustacean plus expérimenté pourrait écrire ce code à la place du précédent :

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problème lors de la création du fichier : {:?}", error);
            })
        } else {
            panic!("Problème lors de l'ouverture du fichier : {:?}", error);
        }
    });
}
```

Bien que ce code ait le même comportement que le précédent, il ne contient aucune expression `match` et est plus lisible. Revenez à cet exemple après avoir lu ["Types de bibliothèque standard/Closures"](course://Standard Library Types/Closures), et recherchez la méthode `unwrap_or_else` dans la documentation de la bibliothèque standard. Beaucoup d'autres de ces méthodes peuvent simplifier de grandes expressions `match` imbriquées lorsque vous gérez des erreurs.