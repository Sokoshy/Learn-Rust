### Utiliser Result<T, E> dans les tests

Jusqu'à présent, nous avons écrit des tests qui déclenchent une panique lorsqu'ils échouent. Nous pouvons également écrire des tests qui utilisent `Result<T, E>` ! Voici le test d'[une des tâches précédentes](course://Writing Automated Tests/How to Write Tests/The Anatomy of a Test Function), réécrit pour utiliser `Result<T, E>` et retourner une `Err` au lieu de panique :

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("deux plus deux n'égale pas quatre"))
            }
        }
    }
```

La fonction `it_works` a maintenant un type de retour, `Result<(), String>`. Dans le corps de la fonction, plutôt que d'appeler la macro `assert_eq!`, nous retournons `Ok(())` lorsque le test réussit et une `Err` avec une `String` à l'intérieur lorsque le test échoue.

Écrire des tests de manière à ce qu'ils retournent un `Result<T, E>` vous permet d'utiliser l'opérateur point d'interrogation dans le corps des tests, ce qui peut être une manière pratique d'écrire des tests qui devraient échouer si une opération à l'intérieur d'eux retourne une variante `Err`.

Vous ne pouvez pas utiliser l'annotation `#[should_panic]` sur des tests qui utilisent `Result<T, E>`. Au lieu de cela, vous devez retourner directement une valeur `Err` lorsque le test doit échouer.

Maintenant que vous connaissez plusieurs façons d'écrire des tests, examinons ce qui se passe lorsque nous exécutons nos tests et explorons les différentes options que nous pouvons utiliser avec `cargo test`.