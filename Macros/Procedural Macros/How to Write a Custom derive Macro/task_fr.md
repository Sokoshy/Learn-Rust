### Comment écrire une macro `derive` personnalisée

Créons une crate nommée `hello_macro` qui définit un trait nommé `HelloMacro` avec une fonction associée nommée `hello_macro`. Plutôt que d'obliger nos utilisateurs de crate à implémenter le trait `HelloMacro` pour chacun de leurs types, nous fournirons une macro procédurale afin que les utilisateurs puissent annoter leur type avec `#[derive(HelloMacro)]` pour obtenir une implémentation par défaut de la fonction `hello_macro`. L'implémentation par défaut affichera `Hello, Macro! My name is TypeName!` où `TypeName` est le nom du type pour lequel ce trait a été défini. En d'autres termes, nous écrirons une crate qui permet à un autre programmeur d'écrire un code comme celui ci-dessous en utilisant notre crate.

```rust
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    fn main() {
        Pancakes::hello_macro();
    }
```

##### Le code qu'un utilisateur de notre crate pourra écrire en utilisant notre macro procédurale

Ce code affichera `Hello, Macro! My name is Pancakes!` lorsque nous aurons terminé.