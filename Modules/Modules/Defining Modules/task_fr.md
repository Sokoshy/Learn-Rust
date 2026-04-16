## Définir des modules pour contrôler la portée et la confidentialité

Les _modules_ nous permettent d'organiser le code dans une crate en groupes pour faciliter la lisibilité et la réutilisation. Les modules contrôlent également la _confidentialité_ des éléments, c'est-à-dire si un élément peut être utilisé par du code externe (_public_) ou s'il s'agit d'un détail d'implémentation interne non disponible pour un usage externe (_privé_).

À titre d'exemple, écrivons une crate de bibliothèque qui fournit les fonctionnalités d'un restaurant. Nous allons définir les signatures des fonctions mais laisser leurs corps vides pour nous concentrer sur l'organisation du code, plutôt que de réellement implémenter un restaurant en code.

Dans le secteur de la restauration, certaines parties d'un restaurant sont appelées _salle_ et d'autres _cuisine_. La salle est l'endroit où se trouvent les clients ; c'est là que les hôtes installent les clients, les serveurs prennent les commandes et les paiements, et les barmans préparent les boissons. La cuisine est là où les chefs et cuisiniers travaillent, les plongeurs nettoient, et les gestionnaires effectuent des tâches administratives.

Pour structurer notre crate de la même manière qu'un restaurant réel, nous pouvons organiser les fonctions en modules imbriqués. Créez une nouvelle bibliothèque nommée `restaurant` en exécutant `cargo new --lib restaurant`; puis mettez le code de l'exemple ci-dessous dans _src/lib.rs_ pour définir certains modules et signatures de fonctions.

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }
```

##### Un module front_of_house contenant d'autres modules qui contiennent ensuite des fonctions

Nous définissons un module en commençant par le mot-clé `mod`, puis nous spécifions le nom du module (dans ce cas, `front_of_house`) et plaçons des accolades autour du corps du module. À l'intérieur des modules, nous pouvons avoir d'autres modules, comme dans ce cas avec les modules `hosting` et `serving`. Les modules peuvent également contenir des définitions pour d'autres éléments, tels que des structures, énumérations, constantes, traits, ou comme dans l'extrait de code ci-dessus, des fonctions.

En utilisant des modules, nous pouvons regrouper des définitions connexes et nommer la raison pour laquelle elles sont reliées. Les programmeurs utilisant ce code auraient plus de facilité à trouver les définitions qu'ils souhaitent utiliser parce qu'ils pourraient naviguer dans le code en fonction des groupes plutôt que d'avoir à lire toutes les définitions. Les programmeurs ajoutant de nouvelles fonctionnalités à ce code sauraient où placer le code pour garder le programme organisé.

Plus tôt, nous avons mentionné que _src/main.rs_ et _src/lib.rs_ sont appelés _racines de crate_. La raison de leur nom est que le contenu de l'un ou l'autre de ces deux fichiers forme un module nommé `crate` à la racine de la structure de modules de la crate, connue sous le nom de _arbre de modules_.

Ci-dessous se trouve l'arbre de modules pour la structure dans l'extrait ci-dessus.

    crate
     └── front_of_house
         ├── hosting
         │   ├── add_to_waitlist
         │   └── seat_at_table
         └── serving
             ├── take_order
             ├── serve_order
             └── take_payment

##### L'arbre de modules

Cet arbre montre comment certains modules s'imbriquent les uns dans les autres (par exemple, `hosting` s'imbrique dans `front_of_house`). L'arbre montre également que certains modules sont _frères_ les uns des autres, ce qui signifie qu'ils sont définis dans le même module (`hosting` et `serving` sont définis dans `front_of_house`). Pour continuer la métaphore familiale, si le module A est contenu à l'intérieur du module B, nous disons que le module A est l'_enfant_ du module B, et que le module B est le _parent_ du module A. Remarquez que l'ensemble de l'arbre de modules est enraciné sous le module implicite nommé `crate`.

L'arbre de modules pourrait vous rappeler l'arborescence des répertoires du système de fichiers sur votre ordinateur; c'est une comparaison très appropriée ! Tout comme les répertoires dans un système de fichiers, vous utilisez des modules pour organiser votre code. Et tout comme les fichiers dans un répertoire, nous avons besoin d'un moyen de trouver nos modules.

_Vous pouvez vous référer au chapitre suivant dans le Livre du Langage de Programmation Rust : [Définir des modules pour contrôler la portée et la confidentialité](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)_