### Tester l'égalité avec les macros assert_eq! et assert_ne!

Une méthode courante pour tester la fonctionnalité consiste à comparer le résultat du code en cours de test à la valeur que vous attendez que le code renvoie pour vous assurer qu'ils sont égaux. Vous pourriez le faire en utilisant la macro `assert!` et en lui passant une expression utilisant l'opérateur `==`. Cependant, c'est un test si courant que la bibliothèque standard fournit une paire de macros—`assert_eq!` et `assert_ne!`—pour effectuer ce test de manière plus pratique. Ces macros comparent deux arguments pour l'égalité ou l'inégalité, respectivement. Elles afficheront également les deux valeurs si l'assertion échoue, ce qui facilite la compréhension de _pourquoi_ le test a échoué ; inversement, la macro `assert!` indique seulement qu'elle a obtenu une valeur `false` pour l'expression `==`, pas les valeurs qui ont conduit à la valeur `false`.

Dans l'extrait de code ci-dessous, nous écrivons une fonction nommée `add_two` qui ajoute `2` à son paramètre et renvoie le résultat. Ensuite, nous testons cette fonction en utilisant la macro `assert_eq!`.

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
```

##### Exemple de test de la fonction add_two en utilisant la macro assert_eq!

Vérifions qu'il passe !

```text
    running 1 test
    test tests::it_adds_two ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Le premier argument que nous avons donné à la macro `assert_eq!`, `4`, est égal au résultat de l'appel à `add_two(2)`. La ligne pour ce test est `test tests::it_adds_two ... ok`, et le texte `ok` indique que notre test a réussi !

Introduisons un bug dans notre code pour voir à quoi cela ressemble lorsqu'un test qui utilise `assert_eq!` échoue. Modifiez l'implémentation de la fonction `add_two` pour ajouter `3` à la place :

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 3
    }
```

Exécutez à nouveau les tests :

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Notre test a attrapé le bug ! Le test `it_adds_two` a échoué, affichant le message `assertion failed: '(left == right)'` et montrant que `left` était `4` et `right` était `5`. Ce message est utile et nous aide à commencer le débogage : cela signifie que l'argument `left` pour `assert_eq!` était `4` mais que l'argument `right`, où nous avions `add_two(2)`, était `5`.

Notez que dans certaines langues et cadres de test, les paramètres des fonctions qui affirment que deux valeurs sont égales sont appelés `expected` et `actual`, et l'ordre dans lequel nous spécifions les arguments est important. Cependant, en Rust, ils sont appelés `left` et `right`, et l'ordre dans lequel nous spécifions la valeur que nous attendons et la valeur que le code en cours de test produit n'a pas d'importance. Nous pourrions écrire l'assertion dans ce test comme `assert_eq!(add_two(2), 4)`, ce qui entraînerait un message d'échec affichant `assertion failed: '(left == right)'` et que `left` était `5` et `right` était `4`.

La macro `assert_ne!` réussira si les deux valeurs que nous lui donnons ne sont pas égales et échouera si elles sont égales. Cette macro est la plus utile pour les cas où nous ne sommes pas sûrs de ce qu'une valeur _sera_, mais nous savons ce que la valeur ne _sera_ certainement pas si notre code fonctionne comme prévu. Par exemple, si nous testons une fonction qui est garantie de modifier son entrée d'une certaine manière, mais que la manière dont l'entrée est modifiée dépend du jour de la semaine où nous exécutons nos tests, la meilleure chose à affirmer pourrait être que la sortie de la fonction n'est pas égale à l'entrée.

En coulisses, les macros `assert_eq!` et `assert_ne!` utilisent les opérateurs `==` et `!=`, respectivement. Lorsque les assertions échouent, ces macros impriment leurs arguments en utilisant le formatage de débogage, ce qui signifie que les valeurs comparées doivent implémenter les traits `PartialEq` et `Debug`. Tous les types primitifs et la plupart des types de la bibliothèque standard implémentent ces traits. Pour les structures et énumérations que vous définissez, vous devrez implémenter `PartialEq` pour affirmer que les valeurs de ces types sont égales ou non égales. Vous devrez implémenter `Debug` pour imprimer les valeurs lorsque l'assertion échoue. Étant donné que les deux traits sont des traits dérivables, comme mentionné dans le listing "Ajout de l'annotation pour dériver le trait `Debug` et impression de l'instance `Rectangle` en utilisant le formatage de débogage" dans le chapitre "Structures", section "Exemples de structures", cela est généralement aussi simple que d'ajouter l'annotation `#[derive(PartialEq, Debug)]` à votre définition de structure ou d'énumération. Voir l'annexe C, [“Traits Dérivables,”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) pour plus de détails sur ces traits et d'autres traits dérivables.