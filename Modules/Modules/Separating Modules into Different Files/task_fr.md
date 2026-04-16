## Séparater les modules dans différents fichiers

Jusqu'à présent, tous les exemples de ce chapitre définissaient plusieurs modules dans un seul fichier. Lorsque les modules deviennent volumineux, vous souhaiterez peut-être déplacer leurs définitions vers un fichier séparé pour faciliter la navigation dans le code.

Par exemple, partons du code dans l'une des tâches précédentes et déplaçons le module `front_of_house` vers son propre fichier _src/front_of_house.rs_ en modifiant le fichier racine du crate de manière à contenir le code ci-dessous. Dans ce cas, le fichier racine du crate est _src/lib.rs_, mais cette procédure fonctionne également avec les crates binaires dont le fichier racine est _src/main.rs_.

```rust
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Déclaration du module front_of_house dont le corps sera dans _src/front_of_house.rs_

Et _src/front_of_house.rs_ obtient les définitions du corps du module `front_of_house`, comme illustré ci-dessous.

<span class="filename">Nom de fichier : src/front_of_house.rs</span>

```rust
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
```

##### Définitions à l'intérieur du module front_of_house dans _src/front_of_house.rs_

L'utilisation d'un point-virgule après `mod front_of_house` plutôt qu'un bloc indique à Rust de charger le contenu du module à partir d'un autre fichier portant le même nom que le module. Pour continuer avec notre exemple et extraire également le module `hosting` vers son propre fichier, nous modifions _src/front_of_house.rs_ pour ne contenir que la déclaration du module `hosting` :

```rust
    pub mod hosting;
```

Ensuite, nous créons un répertoire _src/front_of_house_ et un fichier _src/front_of_house/hosting.rs_ pour contenir les définitions faites dans le module `hosting` :

```rust
    pub fn add_to_waitlist() {}
```

L'arborescence des modules reste la même, et les appels de fonction dans `eat_at_restaurant` fonctionneront sans aucune modification, même si les définitions résident dans différents fichiers. Cette technique vous permet de déplacer des modules dans de nouveaux fichiers à mesure qu'ils prennent de l'ampleur.

Notez que l'instruction `pub use crate::front_of_house::hosting` dans _src/lib.rs_ n’a également pas changé, et que `use` n’a aucun impact sur les fichiers qui sont compilés comme partie du crate. Le mot-clé `mod` déclare des modules, et Rust cherche dans un fichier portant le même nom que le module le code qui entre dans ce module.