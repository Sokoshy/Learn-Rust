## Les corps de fonction contiennent des instructions et des expressions

Les corps de fonction sont composés d'une série d'instructions se terminant éventuellement par une expression. Jusqu'à présent, nous avons seulement couvert des fonctions sans expression de fin, mais vous avez vu une expression dans le cadre d'une instruction. Puisque Rust est un langage basé sur les expressions, cette distinction est importante à comprendre. D'autres langages n'ont pas les mêmes distinctions, alors voyons ce que sont les instructions et les expressions et comment leurs différences affectent les corps de fonction.

Nous avons déjà utilisé des instructions et des expressions. Les _instructions_ sont des commandes qui exécutent une action et ne renvoient pas de valeur. Les _expressions_ évaluent une valeur résultante. Examinons quelques exemples.

Créer une variable et lui attribuer une valeur avec le mot-clé `let` est une instruction. Dans l'exemple ci-dessous, `let y = 6;` est une instruction.

```rust
fn main() {
    let y = 6;
}
```
##### Exemple de déclaration de fonction main contenant une instruction

Les définitions de fonction sont également des instructions ; l'exemple précédent dans son entier est en soi une instruction.

Les instructions ne renvoient pas de valeurs. Ainsi, vous ne pouvez pas assigner une instruction `let` à une autre variable, comme le code suivant essaie de le faire ; vous obtiendrez une erreur :

```rust
fn main() {
    let x = (let y = 6);
}
```

Lorsque vous exécutez ce programme, l'erreur que vous obtiendrez ressemble à ceci :

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

L'instruction `let y = 6` ne renvoie pas de valeur, il n'y a donc rien à attacher à `x`. Cela diffère de ce qui se passe dans d'autres langages, comme C et Ruby, où l'assignation renvoie la valeur de l'assignation. Dans ces langages, vous pouvez écrire `x = y = 6` et avoir `x` et `y` qui ont tous deux la valeur `6`; ce n'est pas le cas en Rust.

Les expressions s'évaluent en quelque chose et constituent la majeure partie du reste du code que vous écrirez en Rust. Considérons une simple opération mathématique, telle que `5 + 6`, qui est une expression qui s'évalue à la valeur `11`. Les expressions peuvent faire partie des instructions : dans le Listing 3-1, le `6` dans l'instruction `let y = 6;` est une expression qui s'évalue à la valeur `6`. Appeler une fonction est une expression. Appeler une macro est une expression. Le bloc que nous utilisons pour créer de nouvelles portées, `{}`, est une expression, par exemple :

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("La valeur de y est : {}", y);
}
```

Cette expression :

```rust
{
    let x = 3;
    x + 1
}
```

est un bloc qui, dans ce cas, s'évalue à `4`. Cette valeur est associée à `y` dans le cadre de l'instruction `let`. Notez la ligne `x + 1` sans point-virgule à la fin, ce qui diffère de la plupart des lignes que vous avez vues jusqu'à présent. Les expressions ne comportent pas de point-virgule final. Si vous ajoutez un point-virgule à la fin d'une expression, vous la transformez en instruction, qui ne renverra alors pas de valeur. Gardez cela à l'esprit lorsque vous explorerez les valeurs de retour de fonction et les expressions ensuite.

_Vous pouvez vous référer à la section suivante du livre de Rust : [Les corps de fonction contiennent des instructions et des expressions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions)._

Faisons maintenant quelques exercices.