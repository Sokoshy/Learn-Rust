## Référence rapide des modules
Voici comment les modules, les chemins, le mot-clé `use` et le mot-clé `pub` fonctionnent dans le compilateur, et comment la plupart des développeurs organisent leur code. Nous passerons en revue des exemples de chacune de ces règles, mais c'est un excellent endroit où revenir à l'avenir pour se rappeler comment les modules fonctionnent.

- **Commencer à partir de la racine du crate** : Lors de la compilation d'un crate, le compilateur regarde d'abord dans le fichier racine du crate (généralement _src/lib.rs_ pour un crate de bibliothèque ou _src/main.rs_ pour un crate binaire).
- **Déclaration de modules** : Dans le fichier racine du crate, vous pouvez déclarer un nouveau module nommé, par exemple, « garden », avec `mod garden;`. Le compilateur recherchera le code à l'intérieur du module dans ces endroits :
  - En ligne, directement après `mod garden`, avec des accolades à la place du point-virgule
  - Dans le fichier _src/garden.rs_
  - Dans le fichier _src/garden/mod.rs_
- **Déclaration de sous-modules** : Dans n'importe quel fichier autre que la racine du crate étant compilé en tant que partie du crate (par exemple, _src/garden.rs_), vous pouvez déclarer des sous-modules (par exemple, `mod vegetables;`). Le compilateur recherchera le code à l'intérieur des sous-modules dans ces endroits à l'intérieur d'un répertoire nommé d'après le module parent :
  - En ligne, directement après `mod vegetables`, avec des accolades à la place du point-virgule
  - Dans le fichier _src/garden/vegetables.rs_
  - Dans le fichier _src/garden/vegetables/mod.rs_
- **Chemins vers le code dans les modules** : Une fois qu'un module est compilé en tant que partie de votre crate, vous pouvez faire référence au code dans ce module (par exemple, un type `Asparagus` dans le module de légumes du jardin) depuis n'importe où ailleurs dans ce crate en utilisant le chemin `crate::garden::vegetables::Asparagus`, tant que les règles de confidentialité le permettent.
- **Privé vs public** : Le code dans un module est privé depuis ses modules parents par défaut. Pour rendre un module public, déclarez-le avec `pub mod` au lieu de `mod`. Pour rendre les éléments dans un module public également public, utilisez `pub` avant leurs déclarations.
- **Le mot-clé `use`** : Dans une portée, le mot-clé `use` crée des raccourcis vers des éléments pour réduire la répétition de chemins longs. Dans toute portée qui peut référencer `crate::garden::vegetables::Asparagus`, vous pouvez créer un raccourci avec `use crate::garden::vegetables::Asparagus;` et ensuite simplement écrire `Asparagus` pour utiliser ce type dans la portée.

Voici un crate binaire nommé `backyard` qui illustre ces règles. Le répertoire du crate, également nommé `backyard`, contient ces fichiers et répertoires :

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Le fichier racine du crate, dans ce cas _src/main.rs_, contient :

Nom du fichier : src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("Je fais pousser {:?}!", plant);
}
```

Le `pub mod garden;` signifie que le compilateur inclut le code qu'il trouve dans _src/garden.rs_, qui est :

Nom du fichier : src/garden.rs

```rust
pub mod vegetables;
```

Et `pub mod vegetables;` signifie que le code dans _src/garden/vegetables.rs_ est également inclus :

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

Passons maintenant aux détails de ces règles et démontrons-les en action !