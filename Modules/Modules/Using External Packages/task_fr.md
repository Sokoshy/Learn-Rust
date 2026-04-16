### Utiliser des packages externes

Supposons que nous voulions obtenir des nombres aléatoires. Pour cela, nous aurons besoin d'un package externe appelé `rand`. Pour utiliser `rand` dans notre projet, nous ajoutons cette ligne à _Cargo.toml_ :

```toml
    [dependencies]
    rand = "0.8.5"
```

Ajouter `rand` comme dépendance dans _Cargo.toml_ indique à Cargo de télécharger le package `rand` et toutes ses dépendances depuis _https://crates.io_ et de rendre `rand` disponible pour notre projet.

Ensuite, pour amener les définitions de `rand` dans le champ de notre package, nous ajoutons une ligne `use` commençant par le nom de la crate, `rand`, et listons les éléments que nous voulons amener dans le champ. Par exemple, nous amenons le trait `Rng` dans le champ et appelons la fonction `rand::thread_rng` :

```rust
    use rand::Rng;
    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }
```

Les membres de la communauté Rust ont mis de nombreux packages à disposition sur _https://crates.io_, et importer l'un d'eux dans votre package implique ces mêmes étapes : les lister dans le fichier _Cargo.toml_ de votre package et utiliser `use` pour amener les éléments de leurs crates dans le champ.

Notez que la bibliothèque standard (`std`) est également une crate externe à notre package. Comme la bibliothèque standard est fournie avec le langage Rust, nous n'avons pas besoin de modifier _Cargo.toml_ pour inclure `std`. Mais nous devons nous y référer avec `use` pour amener des éléments de celle-ci dans le champ de notre package. Par exemple, avec `HashMap`, nous utiliserions cette ligne :

```rust
    use std::collections::HashMap;
```

Ceci est un chemin absolu commençant par `std`, le nom de la crate de la bibliothèque standard.