## Importer des chemins dans la portée avec le mot-clé use

Il pourrait sembler que les chemins que nous avons écrits pour appeler des fonctions jusqu'à présent sont gênants par leur longueur et leur répétition. Par exemple, dans un des exemples de la section "Chemins pour faire référence à un élément dans l'arbre du module", que ce soit en choisissant le chemin absolu ou relatif vers la fonction `add_to_waitlist`, chaque fois que nous voulions appeler `add_to_waitlist`, nous devions également spécifier `front_of_house` et `hosting`. Heureusement, il existe un moyen de simplifier ce processus. Nous pouvons importer un chemin dans une portée une fois et ensuite appeler les éléments de ce chemin comme s'ils étaient des éléments locaux avec le mot-clé `use`.

Dans l'exemple ci-dessous, nous importons le module `crate::front_of_house::hosting` dans la portée de la fonction `eat_at_restaurant`, donc nous n'avons qu'à spécifier `hosting::add_to_waitlist` pour appeler la fonction `add_to_waitlist` dans `eat_at_restaurant`.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Importer un module dans la portée avec use

Ajouter `use` et un chemin dans une portée est similaire à créer un lien symbolique dans le système de fichiers. En ajoutant `use crate::front_of_house::hosting` à la racine du crate, `hosting` est maintenant un nom valide dans cette portée, comme si le module `hosting` avait été défini à la racine du crate. Les chemins importés dans la portée avec `use` vérifient également la confidentialité, comme tous les autres chemins.

Vous pouvez également importer un élément dans la portée avec `use` et un chemin relatif. L'exemple ci-dessous montre comment spécifier un chemin relatif pour obtenir le même comportement qu'avec le code ci-dessus.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use self::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### Importer un module dans la portée avec use et un chemin relatif commençant par self

Notez que l'utilisation de `self` de cette manière pourrait ne pas être nécessaire à l'avenir ; c'est une incohérence dans le langage que les développeurs de Rust travaillent à éliminer.

### Créer des chemins d'utilisation idiomatiques

Dans le premier extrait de code, vous vous êtes peut-être demandé pourquoi nous avons spécifié `use crate::front_of_house::hosting` et ensuite appelé `hosting::add_to_waitlist` dans `eat_at_restaurant` plutôt que de spécifier le chemin `use` jusqu'à la fonction `add_to_waitlist` pour obtenir le même résultat, comme dans l'extrait ci-dessous.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        add_to_waitlist();
        add_to_waitlist();
        add_to_waitlist();
    }
```

##### Importer la fonction add_to_waitlist dans la portée avec use, ce qui n'est pas idiomatique

Bien que les deux extraits accomplissent la même tâche, le premier est la manière idiomatique d'importer une fonction dans la portée avec `use`. Importer le module parent de la fonction dans la portée avec `use` pour que nous devions spécifier le module parent lorsque nous appelons la fonction clarifie que la fonction n'est pas définie localement tout en minimisant la répétition du chemin complet. Le code dans le dernier extrait n'est pas clair quant à l'endroit où `add_to_waitlist` est défini.

D'un autre côté, lorsqu'on importe des structures, des énumérations et d'autres éléments avec `use`, il est idiomatique de spécifier le chemin complet. L'exemple ci-dessous montre la manière idiomatique d'importer la structure `HashMap` de la bibliothèque standard dans la portée d'un crate binaire.

```rust
    use std::collections::HashMap;

    fn main() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
```

##### Importer HashMap dans la portée de manière idiomatique

Il n'y a pas de raison forte derrière cette idiome : c'est juste la convention qui a émergé, et les gens se sont habitués à lire et écrire du code Rust de cette manière.

L'exception à cette idiome est si nous importons deux éléments avec le même nom dans la portée avec des déclarations `use`, car Rust ne le permet pas. L'exemple ci-dessous montre comment importer deux types `Result` dans la portée qui ont le même nom mais des modules parents différents et comment s'y référer.

```rust
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
    }

    fn function2() -> io::Result<()> {
    }
```

##### Importer deux types avec le même nom dans la même portée nécessite d'utiliser leurs modules parents.

Comme vous pouvez le voir, utiliser les modules parents distingue les deux types `Result`. Si nous avions spécifié `use std::fmt::Result` et `use std::io::Result`, nous aurions eu deux types `Result` dans la même portée et Rust n'aurait pas su lequel nous voulions dire lorsque nous utilisions `Result`. Essayez-le et voyez quelle erreur de compilation vous obtenez !