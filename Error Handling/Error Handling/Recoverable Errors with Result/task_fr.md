## Erreurs récupérables avec Result

La plupart des erreurs ne sont pas assez graves pour que le programme s'arrête entièrement. Parfois, lorsqu'une fonction échoue, c'est pour une raison que vous pouvez facilement interpréter et à laquelle vous pouvez répondre. Par exemple, si vous essayez d'ouvrir un fichier et que cette opération échoue parce que le fichier n'existe pas, vous pourriez vouloir créer le fichier plutôt que de terminer le processus.

L'énumération `Result` est définie comme ayant deux variantes, `Ok` et `Err`, comme suit :

```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

Les `T` et `E` sont des paramètres de type génériques : nous discuterons plus en détail des génériques dans la leçon "Types génériques, Traits et Durée de vie". Ce que vous devez savoir pour l'instant, c'est que `T` représente le type de la valeur qui sera retournée en cas de succès au sein de la variante `Ok`, et `E` représente le type de l'erreur qui sera retournée en cas d'échec au sein de la variante `Err`. Comme `Result` a ces paramètres de type génériques, nous pouvons utiliser le type `Result` et les fonctions que la bibliothèque standard a définies dessus dans de nombreuses situations différentes où la valeur de succès et la valeur d'erreur que nous souhaitons retourner peuvent différer.

Appelons une fonction qui retourne une valeur `Result` car la fonction pourrait échouer. Dans l'extrait de code ci-dessous, nous essayons d'ouvrir un fichier.

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");
    }
```

##### ouverture d'un fichier

Comment savons-nous que `File::open` retourne un `Result` ? Nous pourrions consulter la [documentation API de la bibliothèque standard](https://doc.rust-lang.org/std/index.html), ou bien nous pourrions demander au compilateur ! Si nous donnons à `f` une annotation de type que nous savons _ne pas_ être le type de retour de la fonction et essayons ensuite de compiler le code, le compilateur nous dira que les types ne correspondent pas. Le message d'erreur nous dira alors quel est le type de `f`. Essayons ! Nous savons que le type de retour de `File::open` n'est pas de type `u32`, alors changeons la déclaration `let f` comme suit :

```rust
    let f: u32 = File::open("hello.txt");
```

Essayer de compiler maintenant nous donne le résultat suivant :

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<File, std::io::Error>`
```

Cela nous indique que le type de retour de la fonction `File::open` est un `Result<T, E>`. Le paramètre générique `T` a été rempli ici avec le type de la valeur de succès, `std::fs::File`, qui est un descripteur de fichier. Le type utilisé pour `E` dans la valeur d'erreur est `std::io::Error`.

Ce type de retour signifie que l'appel à `File::open` peut réussir et retourner un descripteur de fichier que nous pouvons lire ou écrire. L'appel de fonction peut également échouer : par exemple, le fichier pourrait ne pas exister, ou nous pourrions ne pas avoir les permissions pour accéder au fichier. La fonction `File::open` doit avoir un moyen de nous indiquer si elle a réussi ou échoué et, en même temps, nous donner soit le descripteur de fichier, soit des informations sur l'erreur. Cette information est exactement ce que l'énumération `Result` transmet.

Dans le cas où `File::open` réussit, la valeur dans la variable `f` sera une instance de `Ok` contenant un descripteur de fichier. Dans le cas où cela échoue, la valeur dans `f` sera une instance de `Err` contenant plus d'informations sur le type d'erreur survenu.

Nous devons ajouter au code dans l'extrait de code ci-dessus pour prendre différentes actions selon la valeur que `File::open` retourne. L'extrait de code suivant montre une façon de gérer le `Result` en utilisant un outil de base, l'expression `match` que nous avons discutée dans ["L'opérateur Match"](course://Enums/Enums and Pattern Matching/The Match Operator).

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problème d'ouverture du fichier : {:?}", error),
    };
}
```

##### utilisation d'une expression match pour gérer les variantes de Result qui pourraient être retournées

Notez que, comme l'énumération `Option`, l'énumération `Result` et ses variantes ont été importées dans la portée par le prélude, donc nous n'avons pas besoin de spécifier `Result::` avant les variantes `Ok` et `Err` dans les bras du `match`.

Ici, nous indiquons à Rust que lorsque le résultat est `Ok`, retourner la valeur `file` interne de la variante `Ok`, puis nous assignons cette valeur de descripteur de fichier à la variable `f`. Après le `match`, nous pouvons utiliser le descripteur de fichier pour la lecture ou l'écriture.

L'autre branche du `match` gère le cas où nous obtenons une valeur `Err` de `File::open`. Dans cet exemple, nous avons choisi d'appeler la macro `panic!`. S'il n'y a pas de fichier nommé _hello.txt_ dans notre répertoire actuel et que nous exécutons ce code, nous verrons la sortie suivante de la macro `panic!` :

```text
thread 'main' panicked at 'Problème d'ouverture du fichier : Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

Comme d'habitude, cette sortie nous indique exactement ce qui n'a pas fonctionné.