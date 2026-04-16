### Utiliser une rétrotrace `panic!`

Examinons un autre exemple pour voir ce qui se passe lorsqu'un appel `panic!` provient d'une bibliothèque à cause d'un bug dans notre code au lieu de notre code appelant directement la macro. Le fragment de code ci-dessous contient un code qui tente d'accéder à un élément par index dans un vecteur.

```rust
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
```

##### Tenter d'accéder à un élément au-delà de la fin d'un vecteur, ce qui entraînera un appel à panic!

Ici, nous essayons d'accéder au 100e élément de notre vecteur (qui est à l'index 99 puisque la numérotation commence à zéro), mais il n'a que 3 éléments. Dans cette situation, Rust va paniquer. Utiliser `[]` est censé retourner un élément, mais si vous passez un index invalide, il n'y a aucun élément que Rust pourrait retourner ici qui serait correct.

D'autres langages, comme C, tenteront de vous donner exactement ce que vous avez demandé dans cette situation, même si ce n'est pas ce que vous voulez : vous obtiendrez ce qu'il y a à l'emplacement en mémoire qui correspondrait à cet élément dans le vecteur, même si la mémoire n'appartient pas au vecteur. Cela s'appelle une _lecture excessive de tampon_ et peut mener à des vulnérabilités de sécurité si un attaquant parvient à manipuler l'index de telle sorte qu'il puisse lire des données auxquelles il ne devrait pas avoir accès, stockées après le tableau.

Pour protéger votre programme d'un tel type de vulnérabilité, si vous essayez de lire un élément à un index qui n'existe pas, Rust arrêtera l'exécution et refusera de continuer. Essayons et voyons :

```text
   Compiling test_rust_project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/test_rust_project`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2686:10
```
Cette erreur pointe vers un fichier que nous n'avons pas écrit, _libcore/slice/mod.rs_. C'est l'implémentation de `slice` dans le code source de Rust. Le code qui s'exécute lorsque nous utilisons `[]` sur notre vecteur `v` est dans _libcore/slice/mod.rs_, et c'est là que le `panic!` se produit réellement.

La prochaine ligne de note nous indique que nous pouvons définir la variable d'environnement `RUST_BACKTRACE` pour obtenir une rétrotrace de ce qui s'est exactement passé pour causer l'erreur. Une _rétrotrace_ est une liste de toutes les fonctions qui ont été appelées pour arriver à ce point. Les rétrotraces dans Rust fonctionnent comme dans d'autres langages : la clé pour lire la rétrotrace est de commencer par le haut et lire jusqu'à voir des fichiers que vous avez écrits. C'est l'endroit où le problème a commencé. Les lignes au-dessus des lignes mentionnant vos fichiers sont du code que votre code a appelé ; les lignes en dessous sont le code qui a appelé votre code. Ces lignes peuvent inclure du code Rust de base, du code de librairie standard ou des bibliothèques que vous utilisez. Essayons d'obtenir une rétrotrace en définissant la variable d'environnement `RUST_BACKTRACE` sur n'importe quelle valeur sauf 0. Le fragment de code ci-dessous montre une sortie similaire à ce que vous verrez.

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

##### La rétrotrace générée par un appel à panic! affichée lorsque la variable d'environnement `RUST_BACKTRACE` est définie

C'est beaucoup de sortie ! La sortie exacte que vous voyez peut différer selon votre système d'exploitation et la version de Rust. Pour obtenir des rétrotraces avec ces informations, les symboles de débogage doivent être activés. Les symboles de débogage sont activés par défaut lors de l'utilisation de `cargo build` ou `cargo run` sans le flag `--release`, comme nous l'avons fait ici.

Dans la sortie du fragment de code ci-dessus, la ligne 6 de la rétrotrace pointe vers la ligne de notre projet qui cause le problème : la ligne 4 de _src/main.rs_. Si nous ne voulons pas que notre programme panique, l'endroit indiqué par la première ligne mentionnant un fichier que nous avons écrit est là où nous devrions commencer notre enquête. Dans le premier fragment de code, où nous avons délibérément écrit du code qui provoquerait une panique afin de démontrer comment utiliser les rétrotraces, la façon de corriger la panique est de ne pas demander un élément à l'index 99 d'un vecteur qui ne contient que 3 éléments. Quand votre code panique à l'avenir, vous devrez déterminer quelle action le code prend avec quelles valeurs pour provoquer la panique et ce que le code devrait faire à la place.

Nous reviendrons sur `panic!` et quand nous devrions et ne devrions pas utiliser `panic!` pour gérer les conditions d'erreur dans la section [“To `panic!` or Not to `panic!`”](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic) plus tard dans ce chapitre. Ensuite, nous verrons comment se remettre d'une erreur en utilisant `Result`.

_Vous pouvez vous référer au chapitre suivant dans le livre du langage de programmation Rust :
[Erreurs irréversibles avec panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)_