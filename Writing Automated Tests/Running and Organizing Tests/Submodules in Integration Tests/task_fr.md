## Sous-modules dans les tests d'intégration

À mesure que vous ajoutez plus de tests d'intégration, vous pourriez vouloir créer plus d'un fichier dans le répertoire *tests* pour vous aider à les organiser; par exemple, vous pouvez regrouper les fonctions de test par la fonctionnalité qu'elles testent. Comme mentionné précédemment, chaque fichier dans le répertoire *tests* est compilé comme une crate distincte.

Traiter chaque fichier de test d'intégration comme une crate distincte est utile pour créer des espaces de noms séparés qui ressemblent plus à la manière dont les utilisateurs finaux utiliseront votre crate. Cependant, cela signifie que les fichiers du répertoire *tests* ne partagent pas le même comportement que les fichiers dans *src*, comme vous l'avez appris dans "Modules et Macros/Modules" concernant comment séparer le code en modules et fichiers.

Le comportement différent des fichiers dans le répertoire *tests* est le plus notable lorsque vous avez un ensemble de fonctions utilitaires qui seraient utiles dans plusieurs fichiers de test d'intégration et que vous essayez de suivre les étapes de la section “Séparer les modules en fichiers différents” de "Modules et Macros/Modules" pour les extraire dans un module commun. Par exemple, si nous créons *tests/common.rs* et plaçons une fonction nommée `setup` dedans, nous pouvons ajouter du code à `setup` que nous voulons appeler depuis plusieurs fonctions de test dans plusieurs fichiers de test :

```rust
pub fn setup() {
    // le code d'installation spécifique aux tests de votre librairie irait ici
}
```

Lorsque nous exécutons à nouveau les tests, nous verrons une nouvelle section dans le résultat des tests pour le fichier *common.rs*, même si ce fichier ne contient aucune fonction de test et que nous n'avons pas appelé la fonction `setup` de nulle part :

```text
   Compiling test_organization v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running target/debug/deps/test_organization-61f5d8d60ccbcc19

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b5e4eefa9d201089

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-5843d720c5feeb7a

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests test_organization

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Voir `common` apparaître dans les résultats de test avec `running 0 tests` affiché pour lui n'est pas ce que nous voulions. Nous voulions simplement partager du code avec les autres fichiers de test d'intégration. Vous apprendrez comment éviter que `common` apparaisse dans les résultats de test et à organiser correctement les tests dans la section suivante.

Pour éviter que `common` apparaisse dans les résultats de test, au lieu de créer *tests/common.rs*, nous allons créer *tests/common/mod.rs*. C'est une convention de nommage alternative que Rust comprend également. Nommer le fichier de cette manière indique à Rust de ne pas traiter le module `common` comme un fichier de test d'intégration. Lorsque nous déplaçons le code de la fonction `setup` dans *tests/common/mod.rs* et supprimons le fichier *tests/common.rs*, la section dans le résultat des tests n'apparaîtra plus. Les fichiers dans les sous-répertoires du répertoire *tests* ne sont pas compilés comme des crates séparées ou n'ont pas de sections dans le résultat des tests.

Après avoir créé *tests/common/mod.rs*, nous pouvons l'utiliser de n'importe quel fichier de test d'intégration comme un module. Voici un exemple d'appel de la fonction `setup` depuis le test `it_adds_two` dans *tests/integration_test.rs* :

```rust,ignore
use test_organization_part_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization_part_2::add_two(2));
}
```

Notez que la déclaration `mod common;` est la même que la déclaration du module que nous avons démontrée dans l'annonce "Déclarer le module front_of_house dont le corps sera dans _src/front_of_house.rs" dans la section "Séparer les modules en fichiers différents" de "Modules". Ensuite, dans la fonction de test, nous pouvons appeler la fonction `common::setup()`.

Résultat de `cargo test` après avoir créé *tests/common/mod.rs* et appelé la fonction `setup` depuis le test `it_adds_two` dans *tests/integration_test.rs* :

```text
Compiling submodules v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running target/debug/deps/submodules-c44b35b673c8053d

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-31048908068047a2

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests submodules

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Tests d'intégration pour les crates binaires

Si notre projet est une crate binaire qui ne contient qu'un fichier *src/main.rs* et n'a pas de fichier *src/lib.rs*, nous ne pouvons pas créer de tests d'intégration dans le répertoire *tests* et importer des fonctions définies dans le fichier *src/main.rs* avec une instruction `use`. Seules les crates de bibliothèque exposent des fonctions que d'autres crates peuvent utiliser; les crates binaires sont conçues pour être exécutées seules.

C'est l'une des raisons pour lesquelles les projets Rust qui fournissent un binaire ont un fichier *src/main.rs* simple qui appelle la logique résidant dans le fichier *src/lib.rs*. En utilisant cette structure, les tests d'intégration peuvent tester la crate de bibliothèque avec `use` pour rendre la fonctionnalité essentielle disponible. Si la fonctionnalité essentielle fonctionne, la petite quantité de code dans le fichier *src/main.rs* fonctionnera également, et cette petite quantité de code n'a pas besoin d'être testée.