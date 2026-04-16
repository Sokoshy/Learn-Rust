## Gérer plusieurs conditions avec else if

Vous pouvez avoir plusieurs conditions en combinant `if` et `else` dans une expression `else if`. Par exemple :

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("le nombre est divisible par 4");
    } else if number % 3 == 0 {
        println!("le nombre est divisible par 3");
    } else if number % 2 == 0 {
        println!("le nombre est divisible par 2");
    } else {
        println!("le nombre n'est pas divisible par 4, 3, ou 2");
    }
}
```

Ce programme a quatre chemins possibles qu'il peut emprunter. Après l'avoir exécuté, vous devriez voir la sortie suivante :

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
le nombre est divisible par 3
```

Lorsque ce programme s'exécute, il vérifie chaque expression `if` à son tour et exécute le premier bloc pour lequel la condition est vraie. Notez que même si 6 est divisible par 2, nous ne voyons pas la sortie 'le nombre est divisible par 2', ni le texte 'le nombre n'est pas divisible par 4, 3, ou 2' du bloc `else`. Cela est dû au fait que Rust exécute uniquement le bloc pour la première condition vraie et, une fois qu'il en trouve une, il ne vérifie même pas le reste.

Utiliser trop d'expressions `else if` peut encombrer votre code, donc si vous en avez plus d'une, vous pourriez vouloir refactoriser votre code. Le [Chapitre 6](https://doc.rust-lang.org/stable/book/ch06-00-enums.html) décrit une construction de branchement puissante en Rust appelée `match` pour ces cas.

_Vous pouvez vous référer au chapitre suivant dans le livre du Langage de Programmation Rust : [Gérer plusieurs conditions avec else if](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#handling-multiple-conditions-with-else-if)_