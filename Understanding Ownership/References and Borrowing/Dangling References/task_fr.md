## Références Dangling

Dans les langages avec pointeurs, il est facile de créer par erreur un _dangling pointer_, un pointeur qui référence un emplacement en mémoire qui peut avoir été attribué à quelqu'un d'autre, en libérant de la mémoire tout en conservant un pointeur vers cette mémoire. En Rust, par contre, le compilateur garantit que les références ne seront jamais des références invalides : si vous avez une référence à certaines données, le compilateur s'assurera que ces données ne sortiront pas du champ avant la référence aux données.

Essayons de créer une référence invalide, que Rust empêchera avec une erreur à la compilation :

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Voici l'erreur :

```text
    error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ^^^^^^^^

```

Ce message d'erreur fait référence à une fonctionnalité que nous n'avons pas encore abordée : les durées de vie. Nous discuterons des durées de vie en détail au chapitre 10. Mais, si vous faites abstraction des parties concernant les durées de vie, le message contient la clé pour comprendre pourquoi ce code pose problème :

```text
    this function's return type contains a borrowed value, but there is no value
    for it to be borrowed from.
```

Examinons de plus près ce qui se passe exactement à chaque étape de notre code `dangle` :

```rust
fn dangle() -> &String { // dangle renvoie une référence à une String

    let s = String::from("hello"); // s est une nouvelle String

    &s // nous renvoyons une référence à la String, s
} // Ici, s sort de la mémoire et est supprimé. Sa mémoire disparaît.
  // Danger!
```

Parce que `s` est créé à l'intérieur de `dangle`, lorsque le code de `dangle` est terminé, `s` sera désalloué. Mais nous avons essayé de renvoyer une référence à `s`. Cela signifie que cette référence pointerait vers une `String` invalide. Ce n'est pas bon ! Rust ne nous permettra pas de faire cela.

La solution ici est de retourner directement la `String` :

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

Cela fonctionne sans aucun problème. La propriété est transférée, et rien n'est désalloué.