### Vérification des paniques avec `should_panic`

En plus de vérifier que notre code renvoie les valeurs correctes que nous attendons, il est également important de vérifier que notre code gère les conditions d'erreur comme prévu. Par exemple, considérons le type `Guess` que nous avons créé dans la section "Paniquer ou ne pas paniquer" du chapitre "Erreurs récupérables et irrécupérables", énumérant "Un type `Guess` qui ne continuera qu'avec des valeurs comprises entre 1 et 100". Un autre code qui utilise `Guess` dépend de la garantie que les instances de `Guess` contiendront uniquement des valeurs entre 1 et 100. Nous pouvons écrire un test qui s'assure que tenter de créer une instance de `Guess` avec une valeur en dehors de cette plage provoque une panique.

Nous le faisons en ajoutant un autre attribut, `should_panic`, à notre fonction de test. Cet attribut fait réussir un test si le code à l'intérieur de la fonction panique ; le test échouera si le code à l'intérieur de la fonction ne panique pas.

L'extrait de code ci-dessous montre un test qui vérifie que les conditions d'erreur de `Guess::new` se produisent lorsque nous les attendons.

```rust
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("La valeur de Guess doit être comprise entre 1 et 100, reçu {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### Exemple de test qu'une condition provoquera une `panic!`

Nous plaçons l'attribut `#[should_panic]` après l'attribut `#[test]` et avant la fonction de test à laquelle il s'applique. Regardons le résultat lorsque ce test réussit :

```text
    running 1 test
    test tests::greater_than_100 ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Cela semble parfait ! Introduisons maintenant un bug dans notre code en supprimant la condition selon laquelle la fonction `new` panique si la valeur est supérieure à 100 :

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1  {
                panic!("La valeur de Guess doit être comprise entre 1 et 100, reçu {}.", value);
            }

            Guess {
                value
            }
        }
    }
```

Lorsque nous exécutons le test dans "Exemple de test qu'une condition provoquera une `panic!`", il échouera :

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: le test n'a pas paniqué comme prévu

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Nous n'obtenons pas un message très utile dans ce cas, mais lorsque nous regardons la fonction de test, nous voyons qu'elle est annotée avec `#[should_panic]`. L'échec que nous avons rencontré signifie que le code dans la fonction de test n'a pas causé de panique.

Les tests qui utilisent `should_panic` peuvent être imprécis car ils indiquent uniquement que le code a causé une certaine panique. Un test `should_panic` réussirait même si le test panique pour une raison différente de celle que nous attendions. Pour rendre les tests `should_panic` plus précis, nous pouvons ajouter un paramètre `expected` optionnel à l'attribut `should_panic`. Le programme de test vérifiera que le message d'échec contient le texte fourni. Par exemple, considérez le code modifié pour `Guess` (ci-dessous) où la fonction `new` panique avec des messages différents selon que la valeur est trop petite ou trop grande.

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("La valeur de Guess doit être supérieure ou égale à 1, reçu {}.", value);
            } else if value > 100 {
                panic!("La valeur de Guess doit être inférieure ou égale à 100, reçu {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "La valeur de Guess doit être inférieure ou égale à 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### Exemple de test qu'une condition provoquera une panic! avec un message de panique particulier

Ce test réussira parce que la valeur que nous avons mise dans le paramètre `expected` de l'attribut `should_panic` est une sous-chaîne du message avec lequel la fonction `Guess::new` panique. Nous aurions pu spécifier l'intégralité du message de panique que nous attendons, qui dans ce cas serait `La valeur de Guess doit être inférieure ou égale à 100, reçu 200.` Ce que vous choisissez de spécifier dans le paramètre expected pour `should_panic` dépend de la quantité de message de panique qui est unique ou dynamique et de la précision que vous souhaitez pour votre test. Dans ce cas, une sous-chaîne du message de panique est suffisante pour garantir que le code de la fonction de test exécute le cas `else if value > 100`.

Pour voir ce qui se passe lorsqu'un test `should_panic` avec un message `expected` échoue, introduisons à nouveau un bug dans notre code en échangeant le corps des blocs `if value < 1` et `else if value > 100` :

```rust
    if value < 1 {
        panic!("La valeur de Guess doit être inférieure ou égale à 100, reçu {}.", value);
    } else if value > 100 {
        panic!("La valeur de Guess doit être supérieure ou égale à 1, reçu {}.", value);
    }
```

Cette fois, lorsque nous exécutons le test `should_panic`, il échouera :

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'main' panicked at 'La valeur de Guess doit être supérieure ou égale à 1, reçu 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"La valeur de Guess doit être supérieure ou égale à 1, reçu 200."`,
 expected substring: `"La valeur de Guess doit être inférieure ou égale à 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Le message d'échec indique que ce test a effectivement paniqué comme nous l'attendions, mais le message de panique ne contenait pas la chaîne attendue `La valeur de Guess doit être inférieure ou égale à 100`. Le message de panique que nous avons obtenu dans ce cas était `La valeur de Guess doit être supérieure ou égale à 1, reçu 200.` Nous pouvons maintenant commencer à comprendre où se trouve notre bug !