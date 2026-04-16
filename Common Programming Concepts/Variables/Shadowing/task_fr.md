## Masquage

En Rust, vous pouvez déclarer une nouvelle variable avec le même nom qu'une variable précédente, et la nouvelle variable masque la variable précédente. Les Rustacés disent que la première variable est _masquée_ par la seconde, ce qui signifie que la valeur de la seconde variable est celle qui apparaît lorsque la variable est utilisée. Nous pouvons masquer une variable en utilisant le même nom de variable et en répétant l'utilisation du mot-clé `let` comme suit :

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("La valeur de x est : {}", x);
}
```

Ce programme lie d'abord `x` à une valeur de `5`. Ensuite, il masque `x` en répétant `let x =`, prenant la valeur originale et ajoutant `1` de sorte que la valeur de `x` soit ensuite `6`. La troisième déclaration `let` masque également `x`, multipliant la valeur précédente par `2` pour donner à `x` une valeur finale de `12`. Lorsque nous exécutons ce programme, il affichera ce qui suit :

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
La valeur de x est : 12
```

Le masquage est différent de la qualification d'une variable avec `mut`, car nous obtiendrons une erreur de compilation si nous essayons par erreur de réassigner cette variable sans utiliser le mot-clé `let`. En utilisant `let`, nous pouvons effectuer quelques transformations sur une valeur mais rendre la variable immuable après que ces transformations ont été complétées.

L'autre différence entre `mut` et le masquage est que, parce que nous créons effectivement une nouvelle variable lorsque nous utilisons à nouveau le mot-clé `let`, nous pouvons changer le type de la valeur tout en réutilisant le même nom. Par exemple, supposons que notre programme demande à un utilisateur de montrer combien d'espaces ils veulent entre du texte en entrant des caractères d'espacement, mais nous voulons réellement stocker cette entrée en tant que nombre :

```rust
let spaces = "   ";
let spaces = spaces.len();
```

Cette construction est autorisée parce que la première variable `spaces` est de type chaîne de caractères et la seconde variable `spaces`, qui est une toute nouvelle variable qui a le même nom que la première, est de type nombre. Le masquage nous évite ainsi d'avoir à trouver des noms différents, comme `spaces_str` et `spaces_num`; à la place, nous pouvons réutiliser le nom plus simple spaces. Cependant, si nous essayons d'utiliser mut pour cela, comme montré ici, nous obtiendrons une erreur de compilation :

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

L'erreur indique que nous ne sommes pas autorisés à muter le type d'une variable :

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ type attendu `&str`, trouvé `usize`
  |
  = note: type attendu `&str`
             type trouvé `usize`
```

_Vous pouvez vous référer au chapitre suivant du livre de langue de programmation Rust : [Shadowing](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing)_

Maintenant que nous avons exploré le fonctionnement des variables, appliquons nos connaissances en pratique.