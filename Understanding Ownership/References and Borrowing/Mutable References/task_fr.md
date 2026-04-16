## Références Mutables

Que se passe-t-il si nous essayons de modifier quelque chose que nous empruntons ? Essayez le code dans l'extrait de code ci-dessous. Alerte spoiler : cela ne fonctionne pas !

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

##### Tentative de modification d'une valeur empruntée

Voici l'erreur :

```text
    error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Tout comme les variables sont immuables par défaut, les références le sont aussi. Nous ne sommes pas autorisés à modifier quelque chose auquel nous avons une référence.

Nous pouvons corriger l'erreur dans le code de l'extrait ci-dessus avec juste une petite modification :

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Tout d'abord, nous avons dû changer `s` pour qu'il soit `mut`. Ensuite, nous avons dû créer une référence mutable avec `&mut s` et accepter une référence mutable avec `some_string: &mut String`.