## Erreurs irréparables avec panic!

Parfois, de mauvaises choses se produisent dans votre code, et il n'y a rien que vous puissiez y faire. Dans ces cas, Rust dispose de la macro `panic!`. Lorsque la macro `panic!` s'exécute, votre programme va imprimer un message d'erreur, dérouler et nettoyer la pile, puis quitter. Cela se produit le plus souvent lorsqu'un bug de quelque nature a été détecté et qu'il n'est pas clair pour le programmeur comment gérer l'erreur.

> ### Dérouler la pile ou abandonner en réponse à un panic
>
> Par défaut, lorsqu'un panic se produit, le programme commence à _dérober_ la pile, ce qui signifie que Rust remonte dans la pile et nettoie les données de chaque fonction qu'il rencontre. Mais ce retour en arrière et ce nettoyage demandent beaucoup de travail. L'alternative est d'_abandonner_ immédiatement, ce qui termine le programme sans nettoyage. La mémoire que le programme utilisait devra alors être nettoyée par le système d'exploitation. Si dans votre projet vous devez rendre le binaire résultant aussi petit que possible, vous pouvez passer du déroulage à l'abandon lors d'un panic en ajoutant `panic = 'abort'` dans les sections `[profile]` appropriées de votre fichier _Cargo.toml_. Par exemple, si vous voulez abandonner en cas de panic en mode release, ajoutez ceci :
>
>     [profile.release]
>     panic = 'abort'

Essayons d'appeler `panic!` dans un programme simple :

```rust
    fn main() {
        panic!("crash and burn");
    }
```

Quand vous exécutez le programme, vous verrez quelque chose comme ceci :

```text
  Compiling test_rust_project v0.1.0
      Finished dev [unoptimized + debuginfo] target(s) in 0.42s
       Running `target/debug/test_rust_project`
  thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

L'appel à `panic!` provoque le message d'erreur contenu dans les deux dernières lignes. La première ligne montre notre message de panic et l'endroit dans notre code source où le panic s'est produit : _src/main.rs:2:5_ indique que c'est la deuxième ligne, cinquième caractère de notre fichier _src/main.rs_.

Dans ce cas, la ligne indiquée fait partie de notre code, et si nous allons à cette ligne, nous voyons l'appel de la macro `panic!`. Dans d'autres cas, l'appel à `panic!` peut se trouver dans du code que notre code appelle, et le nom de fichier et le numéro de ligne indiqués par le message d'erreur seront le code de quelqu'un d'autre où la macro `panic!` est appelée, non pas la ligne de notre code qui a éventuellement conduit à l'appel `panic!`. Nous pouvons utiliser la trace de la pile des fonctions d'où provient l'appel à `panic!` pour comprendre la partie de notre code qui cause le problème. Nous discuterons plus en détail de ce qu'est une trace de pile ensuite.