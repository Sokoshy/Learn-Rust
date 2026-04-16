### Tests d'intégration

En Rust, les tests d'intégration sont entièrement externes à votre bibliothèque. Ils utilisent votre bibliothèque de la même manière que n'importe quel autre code le ferait, ce qui signifie qu'ils ne peuvent appeler que des fonctions qui font partie de l'API publique de votre bibliothèque. Leur but est de vérifier si plusieurs parties de votre bibliothèque fonctionnent correctement ensemble. Des unités de code qui fonctionnent correctement seules peuvent rencontrer des problèmes lorsqu'elles sont intégrées, donc la couverture de test du code intégré est également importante. Pour créer des tests d'intégration, vous devez d'abord disposer d'un répertoire *tests*.

#### Le répertoire *tests*

Nous avons créé un répertoire *tests* au niveau supérieur de notre répertoire de projet, à côté de *src*. Cargo sait qu'il doit rechercher des fichiers de tests d'intégration dans ce répertoire. Nous pouvons ensuite y créer autant de fichiers de test que nous le souhaitons, et Cargo compilera chacun des fichiers en tant que crate individuel.

Regardons un test d'intégration. Avec le code de l'exemple "Tester une fonction privée" dans le fichier *src/lib.rs*, cherchez dans un répertoire *tests*, où se trouve un fichier nommé *tests/integration_test.rs* avec le code de l'exemple ci-dessous.

```rust
use integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_tests::add_two(2));
}
```

##### Un test d'intégration d'une fonction dans le crate `integration_tests`

Nous avons ajouté `use integration_tests;` en haut du code, ce qui n'était pas nécessaire dans les tests unitaires. La raison en est que chaque fichier dans le répertoire `tests` est une crate distincte, donc nous devons intégrer notre bibliothèque dans la portée de chaque crate de test.

Nous n'avons pas besoin d'annoter le code dans *tests/integration_test.rs* avec `#[cfg(test)]`. Cargo traite le répertoire `tests` de manière spéciale et ne compile les fichiers dans ce répertoire que lorsque nous exécutons `cargo test`. Exécutez `cargo test` maintenant :

```text
Compiling integration_tests v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/integration_tests-61f5d8d60ccbcc19
     
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_tests-d5df7484b111e79e

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests integration_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Les trois sections de sortie incluent les tests unitaires, le test d'intégration et les tests de documentation. La première section pour les tests unitaires est la même que celle que nous avons vue : une ligne pour chaque test unitaire (un nommé `internal` que nous avons ajouté dans "Tester une fonction privée") et ensuite une ligne récapitulative pour les tests unitaires.

La section des tests d'intégration commence par la ligne `Running target/debug/deps/integration_tests-d5df7484b111e79e` (le hash à la fin de votre sortie sera différent). Ensuite, il y a une ligne pour chaque fonction de test dans ce test d'intégration et une ligne récapitulative pour les résultats du test d'intégration juste avant que ne commence la section `Doc-tests integration_tests`.

De manière similaire à comment ajouter plus de fonctions de test unitaires ajoute plus de lignes de résultats dans la section des tests unitaires, ajouter plus de fonctions de test dans le fichier de test d'intégration ajoute plus de lignes de résultats à la section de ce fichier de test d'intégration. Chaque fichier de test d'intégration a sa propre section, donc si nous ajoutons plus de fichiers dans le répertoire *tests*, il y aura plus de sections de tests d'intégration.

Nous pouvons toujours exécuter une fonction de test d'intégration particulière en spécifiant le nom de la fonction de test en tant qu'argument à `cargo test`. Pour exécuter tous les tests dans un fichier de test d'intégration particulier, utilisez l'argument `--test` de `cargo test` suivi du nom du fichier (`cargo test --test integration_test`) :

```text
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Cette commande exécute uniquement les tests dans le fichier *tests/integration_test.rs*.