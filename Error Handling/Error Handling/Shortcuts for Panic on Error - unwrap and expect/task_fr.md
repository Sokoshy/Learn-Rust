### Raccourcis pour panic en cas d'erreur : `unwrap` et `expect`

Utiliser `match` fonctionne assez bien, mais cela peut être un peu verbeux et ne communique pas toujours bien l'intention. Le type `Result<T, E>` a de nombreuses méthodes d'assistance définies pour effectuer diverses tâches. L'une de ces méthodes, appelée `unwrap`, est une méthode raccourcie qui est implémentée exactement comme l'expression `match` que nous avons écrite dans "Utiliser une expression match pour gérer les variantes Result qui pourraient être retournées". Si la valeur `Result` est la variante `Ok`, `unwrap` retournera la valeur à l'intérieur du `Ok`. Si le `Result` est la variante `Err`, `unwrap` appellera la macro `panic!` pour nous. Voici un exemple de `unwrap` en action :

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").unwrap();
    }
```

Si nous exécutons ce code sans le fichier _hello.txt_, nous verrons un message d'erreur provenant de l'appel `panic!` effectué par la méthode `unwrap` :

```text
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
    repr: Os { code: 2, message: "No such file or directory" } }',
    src/libcore/result.rs:906:4
```

Une autre méthode, `expect`, qui est similaire à `unwrap`, nous permet également de choisir le message d'erreur `panic!`. Utiliser `expect` au lieu de `unwrap` et fournir de bons messages d'erreur peut exprimer votre intention et faciliter la recherche de la source d'un panic. La syntaxe de `expect` ressemble à ceci :

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
```

Nous utilisons `expect` de la même manière que `unwrap` : pour retourner le handle de fichier ou appeler la macro `panic!`. Le message d'erreur utilisé par `expect` dans son appel à `panic!` sera le paramètre que nous passons à `expect`, plutôt que le message `panic!` par défaut utilisé par `unwrap`. Voici à quoi cela ressemble :

```text
    thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
    2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Étant donné que ce message d'erreur commence par le texte que nous avons spécifié, `Failed to open hello.txt`, il sera plus facile de trouver d'où provient dans le code ce message d'erreur. Si nous utilisons `unwrap` à plusieurs endroits, cela peut prendre plus de temps pour déterminer exactement quel `unwrap` cause le panic parce que tous les appels `unwrap` qui provoquent un panic impriment le même message.