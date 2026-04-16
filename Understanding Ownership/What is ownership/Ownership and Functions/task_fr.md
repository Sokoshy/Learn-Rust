## Propriété et fonctions

La sémantique pour passer une valeur à une fonction est similaire à celle pour assigner une valeur à une variable. Passer une variable à une fonction la déplacera ou la copiera, tout comme le fait l'assignation. L'exemple ci-dessous contient des annotations montrant où les variables entrent et sortent de la portée.

```rust
fn main() {
    let s = String::from("hello");  // s entre dans la portée

    takes_ownership(s);             // la valeur de s est déplacée dans la fonction...
    // ... et donc n'est plus valide ici

    let x = 5;                      // x entre dans la portée

    makes_copy(x);                  // x serait déplacé dans la fonction,
    // mais i32 est Copy, donc il est toujours possible
    // d'utiliser x ensuite 

} // Ici, x sort de la portée, puis s. Mais comme la valeur de s a été déplacée, rien de
// spécial ne se passe.

fn takes_ownership(some_string: String) { // some_string entre dans la portée
    println!("{}", some_string);
} // Ici, some_string sort de la portée et `drop` est appelé. La mémoire allouée
// est libérée.

fn makes_copy(some_integer: i32) { // some_integer entre dans la portée
    println!("{}", some_integer);
} // Ici, some_integer sort de la portée. Rien de spécial ne se passe.
```

##### Fonctions avec propriété et portée annotées

Si nous essayions d'utiliser `s` après l'appel à `takes_ownership`, Rust générerait une erreur de compilation. Ces vérifications statiques nous protègent des erreurs. Essayez d'ajouter du code dans `main` qui utilise `s` et `x` pour voir où vous pouvez les utiliser et où les règles de propriété vous en empêchent.