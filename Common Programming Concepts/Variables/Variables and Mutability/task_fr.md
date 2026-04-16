## Variables et Mutabilité

Parlons des variables simples.

```rust
let x = 5;
```

Ceci est une instruction `let`, qui est utilisée pour créer une *variable*. Voici un autre exemple :

```rust
let foo = bar;
```

Cette ligne crée une nouvelle variable nommée `foo` et la lie à la valeur de la variable `bar`.

En Rust, par défaut, les variables sont immuables. C'est l'un des nombreux encouragements que Rust vous donne pour que vous écriviez votre code de manière à tirer parti de la sécurité et de la concurrence facile qu'offre Rust. Cependant, vous avez toujours la possibilité de rendre vos variables mutables. Explorons comment et pourquoi Rust vous encourage à favoriser l'immuabilité et pourquoi, parfois, vous pourriez vouloir vous en écarter.

Lorsqu'une variable est immuable, une fois qu'une valeur est liée à un nom, vous ne pouvez pas modifier cette valeur. Regardez le fichier main.rs, son code ne se compilera pas encore :

```rust
fn main() {
       let x = 5;
       println!("La valeur de x est : {}", x);
       x = 6;
       println!("La valeur de x est : {}", x);
   }
```

Exécutez le programme en cliquant sur le bouton **Run**.

Vous devriez recevoir un message d'erreur, comme le montre cette sortie :

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         première affectation à `x`
  |         aide : rendez cette liaison mutable : `mut x`
3 |     println!("La valeur de x est : {}", x);
4 |     x = 6;
  |     ^^^^^ impossible d'affecter deux fois à la variable immuable
```

Cet exemple montre comment le compilateur vous aide à trouver des erreurs dans vos programmes. Même si les erreurs du compilateur peuvent être frustrantes, elles signifient seulement que votre programme ne fait pas encore en toute sécurité ce que vous voulez qu'il fasse ; elles ne signifient pas que vous n'êtes pas un bon programmeur ! Même les développeurs expérimentés en Rust rencontrent des erreurs de compilation.

Le message d'erreur indique que la cause de l'erreur est que vous `ne pouvez pas affecter deux fois à une variable immuable x`, parce que vous avez essayé d'affecter une deuxième valeur à la variable immuable x.

Il est important que nous obtenions des erreurs au moment de la compilation lorsque nous essayons de modifier une valeur que nous avons précédemment désignée comme immuable parce que cette situation peut mener à des bugs. Si une partie de notre code fonctionne en supposant qu'une valeur ne changera jamais et qu'une autre partie de notre code change cette valeur, il est possible que la première partie du code ne fasse pas ce qu'elle était censée faire. La cause de ce type de bug peut être difficile à retrouver après coup, surtout lorsque la deuxième partie du code ne modifie la valeur que parfois.

En Rust, le compilateur garantit que lorsque vous déclarez qu'une valeur ne changera pas, elle ne changera vraiment pas. Cela signifie que lorsque vous lisez et écrivez du code, vous n'avez pas besoin de suivre comment et où une valeur pourrait changer. Votre code est donc plus facile à comprendre.

Mais la mutabilité peut être très utile. Par défaut, les variables sont immuables ; vous pouvez les rendre mutables en ajoutant `mut` devant le nom de la variable. En plus de permettre à cette valeur de changer, `mut` informe les futurs lecteurs du code que d'autres parties du code vont modifier cette valeur de variable.

Par exemple, modifions _src/main.rs_ comme suit :

```rust

fn main() {
    let mut x = 5;
    println!("La valeur de x est : {}", x);
    x = 6;
    println!("La valeur de x est : {}", x);
}
```

Lorsque nous exécutons le programme maintenant, nous obtenons ceci :

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
La valeur de x est : 5
La valeur de x est : 6
```

Nous sommes autorisés à changer la valeur à laquelle `x` est lié de `5` à `6` lorsque `mut` est utilisé. Dans certains cas, vous voudrez rendre une variable mutable parce que cela rend le code plus pratique à écrire que s'il ne comportait que des variables immuables.

Il y a de nombreux compromis à considérer en plus de la prévention des bugs. Par exemple, dans le cas où vous utilisez de grandes structures de données, muter une instance sur place peut être plus rapide que de copier et de renvoyer des instances nouvellement allouées. Avec des structures de données plus petites, créer de nouvelles instances et écrire dans un style de programmation plus fonctionnel peut être plus facile à comprendre, donc des performances moindres peuvent être une pénalité acceptable pour gagner en clarté.

_Vous pouvez vous référer au chapitre suivant dans le livre du Langage de programmation Rust : [Variables et Mutabilité](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)_

Passons maintenant à la tâche pratique.