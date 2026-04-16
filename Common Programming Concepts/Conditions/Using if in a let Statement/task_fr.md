## Utiliser if dans une déclaration let

Puisque `if` est une expression, nous pouvons l'utiliser sur le côté droit d'une déclaration `let`, comme illustré ci-dessous :

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("La valeur de number est : {}", number);
}
```
##### Exemple d'affectation du résultat d'une expression if à une variable

La variable `number` sera liée à une valeur en fonction du résultat de l'expression `if`. Exécutez ce code pour voir ce qui se passe :

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
La valeur de number est : 5
```

Rappelez-vous que les blocs de code sont évalués par l'expression finale qu'ils contiennent, et les nombres eux-mêmes sont également des expressions. Dans ce cas, la valeur de l'ensemble de l'expression `if` dépend de quel bloc de code est exécuté. Cela signifie que les valeurs qui peuvent être le résultat de chaque branche du `if` doivent être du même type ; dans l'exemple précédent, les résultats des deux branches `if` et `else` étaient des entiers `i32`. Si les types ne correspondent pas, comme dans l'exemple suivant, nous obtiendrons une erreur :

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("La valeur de number est : {}", number);
}
```

Lorsque nous essayons de compiler ce code, nous obtiendrons une erreur. Les branches `if` et `else` ont des types de valeurs incompatibles, et Rust indique exactement où trouver le problème dans le programme :

```text
error[E0308]: `if` et `else` ont des types incompatibles
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ attendu entier, trouvée `&str`
  |                                 |
  |                                 attendu à cause de cela
```

L'expression dans le bloc `if` évalue à un entier, et l'expression dans le bloc `else` évalue à une chaîne. Cela ne fonctionne pas car les variables doivent avoir un seul type. Rust doit savoir à la compilation quel est le type de la variable 'number' de manière définitive, afin qu'il puisse vérifier à la compilation que son type est valide partout où nous utilisons 'number'. Rust ne pourrait pas faire cela si le type de 'number' était seulement déterminé à l'exécution ; le compilateur serait plus complexe et offrirait moins de garanties sur le code s'il devait garder trace de types hypothétiques multiples pour n'importe quelle variable.

_Vous pouvez vous référer au chapitre suivant dans le Livre de programmation en Rust : [Utiliser if dans une déclaration let](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#using-if-in-a-let-statement)_