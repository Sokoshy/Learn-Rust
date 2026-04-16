### Exposer des chemins avec le mot-clé pub

Revenons à l'erreur dans l'exemple précédent qui nous disait que le module `hosting` est privé. Nous voulons que la fonction `eat_at_restaurant` dans le module parent ait accès à la fonction `add_to_waitlist` dans le module enfant, alors nous marquons le module `hosting` avec le mot-clé `pub`, comme indiqué dans l'extrait ci-dessous.

```rust
    mod front_of_house {
        pub mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Chemin absolu
        crate::front_of_house::hosting::add_to_waitlist();

        // Chemin relatif
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Déclarer le module hosting comme pub pour l'utiliser depuis eat_at_restaurant

Malheureusement, le code de l'extrait ci-dessus entraîne toujours une erreur comme indiqué ci-dessous.

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: function `add_to_waitlist` is private
 --> src/main.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/main.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^
```

##### Erreurs du compilateur lors de la construction du code ci-dessus

Que s'est-il passé ? Ajouter le mot-clé `pub` devant `mod hosting` rend le module public. Avec ce changement, si nous pouvons accéder à `front_of_house`, nous pouvons accéder à `hosting`. Mais le contenu de `hosting` reste privé ; rendre le module public ne rend pas son contenu public. Le mot-clé `pub` sur un module permet uniquement aux modules ancêtres de se référer à lui.

Les erreurs indiquent que la fonction `add_to_waitlist` est privée. Les règles de visibilité s'appliquent aux structures, énumérations, fonctions et méthodes ainsi qu'aux modules.

Rendons également la fonction `add_to_waitlist` publique en ajoutant le mot-clé `pub` avant sa définition, comme indiqué ci-dessous.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Chemin absolu
        crate::front_of_house::hosting::add_to_waitlist();

        // Chemin relatif
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Ajouter le mot-clé pub à mod hosting et à fn add_to_waitlist nous permet d'appeler la fonction depuis eat_at_restaurant

Maintenant, le code va se compiler ! Examinons le chemin absolu et le chemin relatif et vérifions pourquoi l'ajout du mot-clé `pub` nous permet d'utiliser ces chemins dans `add_to_waitlist` en tenant compte des règles de visibilité.

Dans le chemin absolu, nous commençons par `crate`, la racine de l'arbre des modules de notre crate. Ensuite, le module `front_of_house` est défini à la racine de la crate. Le module `front_of_house` n'est pas public, mais comme la fonction `eat_at_restaurant` est définie dans le même module que `front_of_house` (c'est-à-dire que `eat_at_restaurant` et `front_of_house` sont des modules frères), nous pouvons nous référer à `front_of_house` depuis `eat_at_restaurant`. Ensuite, le module `hosting` est marqué avec `pub`. Nous pouvons accéder au module parent de `hosting`, donc nous pouvons accéder à `hosting`. Enfin, la fonction `add_to_waitlist` est marquée avec `pub` et nous pouvons accéder à son module parent, donc cet appel de fonction fonctionne !

Dans le chemin relatif, la logique est la même que dans le chemin absolu sauf pour la première étape : au lieu de commencer à partir de la racine de la crate, le chemin commence à partir de `front_of_house`. Le module `front_of_house` est défini dans le même module que `eat_at_restaurant`, donc le chemin relatif commençant par le module dans lequel `eat_at_restaurant` est défini fonctionne. Ensuite, comme `hosting` et `add_to_waitlist` sont marqués avec `pub`, le reste du chemin fonctionne et cet appel de fonction est valide !