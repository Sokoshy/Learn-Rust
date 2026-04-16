## Conditions

Décider d'exécuter ou non du code en fonction de si une condition est vraie est un élément de base dans la plupart des langages de programmation. Les constructions les plus courantes qui vous permettent de contrôler le flux d'exécution du code Rust sont les expressions `if`.

### expressions if

Une expression `if` vous permet de bifurquer votre code en fonction de conditions. Vous fournissez une condition, puis vous indiquez : "Si cette condition est remplie, exécutez ce bloc de code. Si la condition n'est pas remplie, n'exécutez pas ce bloc de code."

Dans le fichier _src/main.rs_, saisissez ce qui suit :

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

Toutes les expressions if commencent par le mot-clé `if`, qui est suivi d'une condition. Dans ce cas, la condition vérifie si la variable `number` a une valeur inférieure à 5. Le bloc de code que nous voulons exécuter si la condition est vraie est placé immédiatement après la condition entre accolades. Les blocs de code associés aux conditions dans les expressions `if` sont parfois appelés bras, tout comme les bras dans les expressions `match`.

Optionnellement, nous pouvons également inclure une expression `else`, ce que nous avons choisi de faire ici, pour donner au programme un bloc de code alternatif à exécuter si la condition s'évalue à false. Si vous ne fournissez pas d'expression `else` et que la condition est false, le programme passera simplement le bloc `if` et continuera avec le prochain morceau de code.

Essayez de lancer ce code ; vous devriez voir la sortie suivante :

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

Essayons de changer la valeur de `number` pour une valeur qui rend la condition false pour voir ce qui se passe :

```rust
let number = 7;
```

Exécutez à nouveau le programme et regardez la sortie :

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

Il est également notable que la condition dans ce code doit être un `bool`. Si la condition n'est pas un `bool`, nous obtiendrons une erreur. Par exemple, essayez d'exécuter le code suivant :

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

La condition `if` s'évalue à une valeur de `3` cette fois-ci, et Rust génère une erreur :

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to previous error
```

L'erreur indique que Rust attendait un `bool` mais a trouvé un entier. Contrairement à des langages comme Ruby et JavaScript, Rust ne tentera pas automatiquement de convertir des types non-Boolean en Boolean. Vous devez être explicite et toujours fournir à `if` un Boolean comme condition. Si nous voulons que le bloc de code `if` ne s'exécute que lorsqu'un nombre n'est pas égal à `0`, par exemple, nous pouvons modifier l'expression `if` de la manière suivante :

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

L'exécution de ce code imprimera `number was something other than zero`.

_Vous pouvez vous référer au chapitre suivant du livre The Rust Programming Language : [Le contrôle de flux - les expressions if](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#if-expressions)_