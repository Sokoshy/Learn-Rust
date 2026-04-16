## Ajout de messages d'échec personnalisés

Vous pouvez également ajouter un message personnalisé à afficher avec le message d'échec en tant qu'arguments optionnels aux macros `assert!`, `assert_eq!` et `assert_ne!`. Les arguments spécifiés après l'argument requis pour `assert!` ou les deux arguments requis pour `assert_eq!` et `assert_ne!` sont transmis à la macro `format!` (discutée au chapitre 8 dans la section [“Concaténation avec l'opérateur `+` ou la macro `format!`”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro) du livre), vous pouvez donc passer une chaîne de format contenant des espaces réservés `{}` et des valeurs pour ces espaces réservés. Les messages personnalisés sont utiles pour documenter ce qu'une assertion signifie ; lorsqu'un test échoue, vous aurez une meilleure idée de ce qui ne va pas avec le code.

Par exemple, disons que nous avons une fonction qui salue les gens par leur nom et nous voulons tester que le nom que nous passons dans la fonction apparaît dans le résultat :

```rust
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(result.contains("Carol"));
        }
    }
```

Les exigences pour ce programme ne sont pas encore convenues, et nous sommes assez sûrs que le texte `Hello` au début de la salutation changera. Nous avons décidé que nous ne voulons pas avoir à mettre à jour le test lorsque les exigences changent, donc au lieu de vérifier l'égalité exacte de la valeur retournée par la fonction `greeting`, nous allons simplement vérifier que la sortie contient le texte du paramètre d'entrée.

Introduisons un bug dans ce code en changeant `greeting` pour qu'il n'inclue pas `name` afin de voir à quoi ressemble cet échec de test :

```rust
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
```

L'exécution de ce test produit ce qui suit :

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'assertion failed: result.contains("Carol")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name
```

Ce résultat indique simplement que l'assertion a échoué et sur quelle ligne se trouve l'assertion. Un message d'échec plus utile dans ce cas imprimerait la valeur que nous avons obtenue de la fonction `greeting`. Modifions la fonction de test pour lui donner un message d'échec personnalisé composé d'une chaîne de format avec un espace réservé rempli avec la valeur réelle que nous avons obtenue de la fonction `greeting` :

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

Maintenant, lorsque nous exécutons le test, nous obtiendrons un message d'erreur plus informatif :

```text
---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Nous pouvons voir la valeur que nous avons réellement obtenue dans le résultat du test, ce qui nous aiderait à déboguer ce qui s'est passé au lieu de ce que nous nous attendions à ce qui se produise.