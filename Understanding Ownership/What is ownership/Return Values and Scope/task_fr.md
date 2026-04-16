## Valeurs de retour et portée

Retourner des valeurs peut également transférer la propriété. Le fragment de code ci-dessous est un exemple avec des annotations similaires à celles du fragment précédent.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership déplace sa valeur de retour
    // dans s1

    let s2 = String::from("hello");     // s2 entre dans la portée

    let s3 = takes_and_gives_back(s2);  // s2 est déplacé dans
    // takes_and_gives_back, qui déplace aussi
    // sa valeur de retour dans s3
} // Ici, s3 sort de la portée et est libéré. s2 sort de la portée, mais a été
// déplacé, donc rien ne se passe. s1 sort de la portée et est libéré.

fn gives_ownership() -> String {             // gives_ownership déplacera sa
    // valeur de retour dans la fonction
    // qui l'appelle

    let some_string = String::from("hello"); // some_string entre dans la portée

    some_string                              // some_string est retourné et
    // est déplacé vers la fonction
    // appelante
}

// takes_and_gives_back prendra une String et retournera une String
fn takes_and_gives_back(a_string: String) -> String { // a_string entre dans
    // la portée

    a_string  // a_string est retourné et déplacé vers la fonction appelante
}
```

##### Transférer la propriété des valeurs de retour

La propriété d'une variable suit le même schéma à chaque fois : assigner une valeur à une autre variable la déplace. Lorsqu'une variable contenant des données sur le tas sort de la portée, la valeur sera nettoyée par `drop` à moins que les données n'aient été déplacées pour être possédées par une autre variable.

_Vous pouvez vous référer au chapitre suivant dans le livre The Rust Programming Language : [Propriété](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)_