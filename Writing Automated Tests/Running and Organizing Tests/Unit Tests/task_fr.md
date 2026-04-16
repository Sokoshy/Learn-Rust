### Tests unitaires

Le but des tests unitaires est de tester chaque unité de code de manière isolée du reste du code afin d’identifier rapidement où le code fonctionne et ne fonctionne pas comme prévu. Vous placerez les tests unitaires dans le répertoire *src* dans chaque fichier avec le code qu'ils testent. La convention est de créer un module nommé `tests` dans chaque fichier pour contenir les fonctions de test et d’annoter le module avec `cfg(test)`.

#### Le module tests et `#[cfg(test)]`

L’annotation `#[cfg(test)]` sur le module de tests indique à Rust de compiler et d’exécuter le code de test uniquement lorsque vous exécutez `cargo test`, et non lorsque vous exécutez `cargo build`. Cela permet de gagner du temps de compilation lorsque vous souhaitez uniquement construire la bibliothèque et économise de l’espace dans l’artéfact compilé résultant car les tests ne sont pas inclus. Vous verrez que comme les tests d’intégration se trouvent dans un répertoire différent, ils n’ont pas besoin de l’annotation `#[cfg(test)]`. Cependant, comme les tests unitaires se trouvent dans les mêmes fichiers que le code, vous utiliserez `#[cfg(test)]` pour spécifier qu'ils ne devraient pas être inclus dans le résultat compilé.

Rappelez-vous l’exemple d’un module de test et d’une fonction générés automatiquement par cargo new dans la première section de ce chapitre :

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Ce code est le module de test généré automatiquement. L’attribut `cfg` signifie *configuration* et indique à Rust que l’élément suivant ne doit être inclus que sous une certaine option de configuration. Dans ce cas, l’option de configuration est `test`, qui est fournie par Rust pour compiler et exécuter les tests. En utilisant l’attribut `cfg`, Cargo compile notre code de test uniquement si nous exécutons activement les tests avec `cargo test`. Cela inclut toutes les fonctions auxiliaires qui pourraient se trouver dans ce module, en plus des fonctions annotées avec `#[test]`.

#### Tester les fonctions privées

Il y a un débat au sein de la communauté des tests pour savoir si les fonctions privées doivent être testées directement, et d'autres langages rendent difficile ou impossible le test des fonctions privées. Quelle que soit l’idéologie de test que vous suivez, les règles de confidentialité de Rust vous permettent de tester les fonctions privées. Considérons le code ci-dessous avec la fonction privée `internal_adder`.

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

##### Tester une fonction privée

Notez que la fonction `internal_adder` n’est pas marquée comme `pub`, mais parce que les tests sont simplement du code Rust et que le module `tests` est juste un autre module, vous pouvez intégrer `internal_adder` dans la portée d’un test et l’appeler. Si vous pensez que les fonctions privées ne devraient pas être testées, rien dans Rust ne vous y obligera.