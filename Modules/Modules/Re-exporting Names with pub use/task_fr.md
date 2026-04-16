### Réexporter des noms avec `pub use`

Lorsque nous introduisons un nom dans la portée avec le mot-clé `use`, le nom disponible dans la nouvelle portée est privé. Pour pouvoir se référer à ce nom depuis un autre code (comme si le nom avait été défini dans la portée de ce code), nous pouvons combiner `pub` et `use`. Cette technique s'appelle _réexportation_ car nous introduisons un élément dans la portée tout en rendant cet élément disponible pour que d'autres puissent l'introduire dans leur portée.

L'exemple suivant montre le code du début de la tâche avec `use` dans le module racine changé en `pub use`.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Rendre un nom accessible à tout code à partir d'une nouvelle portée avec pub use

En utilisant `pub use`, le code externe peut désormais appeler la fonction `add_to_waitlist` en utilisant `hosting::add_to_waitlist`. Si nous n'avions pas spécifié `pub use`, la fonction `eat_at_restaurant` pourrait appeler `hosting::add_to_waitlist` dans sa portée, mais le code externe ne pourrait pas profiter de ce nouveau chemin.

La réexportation est utile lorsque la structure interne de votre code est différente de la façon dont les programmeurs utilisant votre code concevraient le domaine. Par exemple, dans cette métaphore du restaurant, les personnes gérant le restaurant pensent en termes de « front of house » et de « back of house ». Mais les clients visitant un restaurant ne penseront probablement pas aux parties du restaurant dans ces termes. Avec `pub use`, nous pouvons écrire notre code avec une structure mais en exposer une différente. Cela rend notre bibliothèque bien organisée pour les programmeurs travaillant sur la bibliothèque et les programmeurs utilisant la bibliothèque.