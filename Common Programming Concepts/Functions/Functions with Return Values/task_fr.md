## Fonctions avec valeurs de retour

Les fonctions peuvent retourner des valeurs au code qui les appelle. Nous ne nommons pas les valeurs de retour, mais nous déclarons leur type après une flèche `(->)`. En Rust, la valeur de retour de la fonction est synonyme de la valeur de la dernière expression dans le bloc de corps d'une fonction. Vous pouvez quitter prématurément une fonction en utilisant le mot-clé `return` et en spécifiant une valeur, mais la plupart des fonctions retournent implicitement la dernière expression. Voici un exemple d'une fonction qui retourne une valeur :

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("La valeur de x est : {}", x);
}
```

Il n'y a pas d'appels de fonction, de macros, ni même d'instructions `let` dans la fonction `five`—juste le nombre 5 par lui-même. C'est une fonction parfaitement valide en Rust. Notez que le type de retour de la fonction est également spécifié, comme `-> i32`. Essayez d'exécuter ce code; la sortie devrait ressembler à ceci :

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
La valeur de x est : 5
```

Le `5` dans `five` est la valeur de retour de la fonction, c'est pourquoi le type de retour est `i32`. Examinons cela en détail. Il y a deux points importants : d'abord, la ligne `let x = five();` montre que nous utilisons la valeur de retour d'une fonction pour initialiser une variable. Parce que la fonction `five` retourne un `5`, cette ligne est équivalente à la suivante :

```rust
let x = 5;
```

Ensuite, la fonction `five` n'a pas de paramètres et définit le type de la valeur de retour, mais le corps de la fonction est un `5` solitaire sans point-virgule car c'est une expression dont nous voulons la valeur pour le retour.

Voyons un autre exemple :

```rust
fn main() {
    let x = plus_one(5);

    println!("La valeur de x est : {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

L'exécution de ce code affichera `La valeur de x est : 6`. Mais si nous ajoutons un point-virgule à la fin de la ligne contenant `x + 1`, la transformant d'une expression en une instruction, nous obtiendrons une erreur.

```rust
fn main() {
    let x = plus_one(5);

    println!("La valeur de x est : {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Compiler ce code produit une erreur, comme suit :

```text
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon
```

Le message d'erreur principal, “types incompatibles”, révèle le problème de base de ce code. La définition de la fonction `plus_one` dit qu'elle retournera un `i32`, mais les instructions n'évaluent pas à une valeur, ce qui est exprimé par `()`, un tuple vide. Par conséquent, rien n'est retourné, ce qui contredit la définition de la fonction et entraîne une erreur. Dans cette sortie, Rust fournit un message pour éventuellement aider à rectifier ce problème : il suggère de retirer le point-virgule, ce qui corrigerait l'erreur.

_Vous pouvez vous référer à la section suivante dans le livre de programmation Rust : [Functions with Return Values](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions-with-return-values)_.