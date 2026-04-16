### Étape 1 : Créer une bibliothèque Crate

La première étape consiste à créer une nouvelle bibliothèque crate. Ici, nous l'avons déjà fait, mais si vous souhaitez l'explorer par vous-même, vous pouvez le faire ainsi :

```text
$ cargo new hello_macro --lib
```

Ensuite, nous allons définir le trait `HelloMacro` et sa fonction associée dans `lib.rs` :

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

Nous avons un trait et sa fonction. À ce stade, l'utilisateur de notre crate pourrait implémenter le trait dans `main.rs` pour obtenir la fonctionnalité souhaitée, comme ceci :

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Bonjour, Macro ! Mon nom est Pancakes !");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

Cependant, ils devraient écrire le bloc d'implémentation pour chaque type qu'ils souhaiteraient utiliser avec `hello_macro` ; nous voulons leur éviter de devoir faire ce travail.

De plus, nous ne pouvons pas encore fournir la fonction `hello_macro` avec une implémentation par défaut qui imprimerait le nom du type sur lequel le trait est implémenté : Rust n'a pas de capacités de réflexion, de sorte qu'il ne peut pas rechercher le nom du type en cours d'exécution. Nous avons besoin d'un macro pour générer du code à la compilation.