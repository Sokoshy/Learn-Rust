## Fonctions

Les fonctions sont omniprésentes dans le code Rust. Vous avez déjà vu l’une des fonctions les plus importantes du langage : la fonction `main`, qui est le point d’entrée de nombreux programmes. Vous avez également vu le mot-clé `fn`, qui vous permet de déclarer de nouvelles fonctions.

Le code Rust utilise le _snake case_ comme style conventionnel pour les noms de fonctions et de variables. En snake case, toutes les lettres sont en minuscules et les mots sont séparés par des tirets bas. Voici un programme qui contient un exemple de définition de fonction :

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Les définitions de fonctions en Rust commencent par `fn` et ont une paire de parenthèses après le nom de la fonction. Les accolades indiquent au compilateur où commence et finit le corps de la fonction.

Nous pouvons appeler n'importe quelle fonction que nous avons définie en entrant son nom suivi par une paire de parenthèses. Étant donné que `another_function` est définie dans le programme, elle peut être appelée à l'intérieur de la fonction `main`. Notez que nous avons défini `another_function` _après_ la fonction `main` dans le code source ; nous aurions pu la définir avant également. Rust ne se soucie pas de l'endroit où vous définissez vos fonctions, seulement qu'elles soient définies quelque part.

Exécutons le code de l'exemple ci-dessus pour explorer les fonctions plus en détail. Placez l'exemple `another_function` dans src/main.rs et exécutez-le. Vous devriez voir la sortie suivante :

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

Les lignes s'exécutent dans l'ordre dans lequel elles apparaissent dans la fonction `main`. D'abord, le message “Hello, world!” s'affiche, puis `another_function` est appelée et son message est imprimé.