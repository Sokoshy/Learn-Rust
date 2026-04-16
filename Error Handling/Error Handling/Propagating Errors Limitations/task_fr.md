#### L'opérateur `?` ne peut être utilisé que dans les fonctions qui renvoient `Result`

L'opérateur `?` ne peut être utilisé que dans les fonctions dont le type de retour est `Result`, car il est défini pour fonctionner de la même manière que l'expression `match` que nous avons créée dans l'exemple avec du code causant une erreur sur match. La partie du `match` qui nécessite un type de retour `Result` est `return Err(e)`, donc le type de retour de la fonction doit être un `Result` pour être compatible avec ce `return`.

Voyons ce qui se passe si nous utilisons l'opérateur `?` dans la fonction `main`, qui, vous vous en souvenez, a un type de retour `()` :

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt")?;
    }
```

Lorsque nous compilons ce code, nous obtenons le message d'erreur suivant :

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `Try` is not implemented for `()`
  = note: required by `from_error`
```

Cette erreur indique que nous ne sommes autorisés à utiliser l'opérateur `?` que dans une fonction qui renvoie `Result` ou `Option` ou un autre type qui implémente `std::ops::Try`. Lorsque vous écrivez du code dans une fonction qui ne retourne pas l'un de ces types et que vous souhaitez utiliser `?` lorsque vous appelez d'autres fonctions qui renvoient `Result<T, E>`, vous avez deux choix pour résoudre ce problème. Une technique consiste à modifier le type de retour de votre fonction pour qu'il soit `Result<T, E>` si aucune restriction ne l'en empêche. L'autre technique est d'utiliser un `match` ou l'une des méthodes `Result<T, E>` pour gérer le `Result<T, E>` de la manière appropriée.

La fonction `main` est spéciale, et il y a des restrictions sur ce que son type de retour doit être. Un type de retour valide pour main est `()`, et heureusement, un autre type de retour valide est `Result<T, E>`, comme illustré ici :

```rust
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
```

Le type `Box<dyn Error>` est appelé un _trait object_, qui est abordé dans la section [« Utiliser des objets de traits qui permettent des valeurs de différents types »](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) du Chapitre 17 du livre. Pour l'instant, vous pouvez lire `Box<dyn Error>` comme signifiant « n'importe quel type d'erreur ». Utiliser `?` dans une fonction `main` avec ce type de retour est autorisé.

Maintenant que nous avons discuté des détails de l'appel de `panic!` ou du retour d'un `Result`, revenons au sujet de la façon de décider lequel est approprié à utiliser dans quels cas.

_Vous pouvez vous référer au chapitre suivant dans le livre du langage de programmation Rust : [Erreurs récupérables avec Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)_