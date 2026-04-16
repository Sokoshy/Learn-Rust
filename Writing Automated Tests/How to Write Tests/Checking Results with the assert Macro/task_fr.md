### Vérification des résultats avec la macro assert!

La macro `assert!`, fournie par la bibliothèque standard, est utile lorsqu'on veut s'assurer qu'une certaine condition dans un test s'évalue à `true`. Nous donnons à la macro `assert!` un argument qui s'évalue en un booléen. Si la valeur est `true`, `assert!` ne fait rien et le test réussit. Si la valeur est `false`, la macro `assert!` appelle la macro `panic!`, ce qui entraîne l'échec du test. Utiliser la macro `assert!` nous aide à vérifier que notre code fonctionne comme nous le souhaitons.

Dans le chapitre "Syntaxe des structures/méthodes", la liste "Implémentation de la méthode `can_hold` sur `Rectangle` qui prend une autre instance de `Rectangle` comme paramètre", nous avons utilisé une structure `Rectangle` et une méthode `can_hold`, qui sont répétées ci-dessous. Mettons ce code dans le fichier _src/lib.rs_ et écrivons quelques tests pour celui-ci en utilisant la macro `assert!`.

```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
```

##### Exemple d'utilisation de la structure `Rectangle` et de sa méthode `can_hold` du chapitre "Syntaxe des structures/méthodes"

La méthode `can_hold` renvoie un booléen, ce qui signifie que c'est un cas d'utilisation parfait pour la macro `assert!`. Dans l'extrait de code ci-dessous, nous écrivons un test qui utilise la méthode `can_hold` en créant une instance de `Rectangle` avec une largeur de 8 et une hauteur de 7 et en vérifiant qu'elle peut contenir une autre instance de `Rectangle` avec une largeur de 5 et une hauteur de 1.

```rust
   #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(larger.can_hold(&smaller));
        }
    }
```

##### Exemple d'un test pour `can_hold` qui vérifie si un rectangle plus grand peut effectivement contenir un rectangle plus petit

Notez que nous avons ajouté une nouvelle ligne à l'intérieur du module `tests` : `use super::*;`. Le module `tests` est un module régulier qui suit les règles de visibilité habituelles que nous avons couvertes dans l'Introduction des "Modules" (chapitre "Modules et macros"). Parce que le module `tests` est un module interne, nous devons faire entrer dans sa portée le code à tester du module externe. Nous utilisons un glob ici pour que tout ce que nous définissons dans le module externe soit disponible pour ce module `tests`.

Nous avons nommé notre test `larger_can_hold_smaller`, et nous avons créé les deux instances de `Rectangle` dont nous avons besoin. Ensuite, nous avons appelé la macro `assert!` et lui avons passé le résultat de l'appel de `larger.can_hold(&smaller)`. Cette expression est censée retourner `true`, donc notre test devrait réussir. Voyons cela !

```text
    running 1 test
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Ça fonctionne ! Ajoutons un autre test, cette fois en vérifiant qu'un rectangle plus petit ne peut pas contenir un rectangle plus grand :

```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            // --snip--
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(!smaller.can_hold(&larger));
        }
    }
```

Parce que le résultat correct de la fonction `can_hold` dans ce cas est `false`, nous devons inverser ce résultat avant de le passer à la macro `assert!`. Ainsi, notre test réussira si `can_hold` retourne `false` :

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Deux tests qui réussissent ! Voyons maintenant ce qui se passe avec nos résultats de test lorsque nous introduisons un bug dans notre code. Changeons l'implémentation de la méthode `can_hold` en remplaçant le signe supérieur par un signe inférieur lorsqu'elle compare les largeurs :

```rust
    // --snip--

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height > other.height
        }
    }
```

Lancer les tests produit maintenant le résultat suivant :

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... FAILED

    failures:

    ---- tests::larger_can_hold_smaller stdout ----
    thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::larger_can_hold_smaller

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Nos tests ont détecté le bug ! Parce que `larger.width` est 8 et `smaller.width` est 5, la comparaison des largeurs dans `can_hold` renvoie maintenant `false` : 8 n'est pas inférieur à 5.