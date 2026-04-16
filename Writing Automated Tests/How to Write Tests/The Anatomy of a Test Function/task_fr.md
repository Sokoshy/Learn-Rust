## Comment écrire des tests

Les tests sont des fonctions Rust qui vérifient que le code non-test fonctionne comme prévu. Les corps des fonctions de test effectuent généralement ces trois actions :

1. Configurer les données ou l'état nécessaires.
2. Exécuter le code que vous voulez tester.
3. Vérifier que les résultats sont conformes à vos attentes.

Voyons les fonctionnalités que Rust propose précisément pour écrire des tests qui réalisent ces actions, comprenant l'attribut `test`, quelques macros, et l'attribut `should_panic`.

### L'anatomie d'une fonction de test

Dans sa forme la plus simple, un test en Rust est une fonction annotée avec l'attribut `test`. Les attributs sont des métadonnées sur des morceaux de code Rust; un exemple est l'attribut `derive` que nous avons utilisé avec les structs dans le chapitre 5. Pour transformer une fonction en fonction de test, ajoutez `#[test]` sur la ligne avant `fn`. Lorsque vous exécutez vos tests avec la commande `cargo test`, Rust construit un binaire de test runner qui exécute les fonctions annotées avec l'attribut `test` et indique si chaque fonction de test réussit ou échoue.

Quand nous créons un nouveau projet de bibliothèque avec Cargo, un module de test avec une fonction de test est automatiquement généré pour nous. Ce module vous aide à commencer à écrire vos tests pour que vous n'ayez pas à rechercher la structure exacte et la syntaxe des fonctions de test chaque fois que vous démarrez un nouveau projet. Vous pouvez ajouter autant de fonctions de test supplémentaires et autant de modules de test que vous le souhaitez !

Nous explorerons certains aspects du fonctionnement des tests en expérimentant avec le modèle de test généré pour nous sans réellement tester de code. Ensuite, nous écrirons quelques tests concrets qui appellent du code que nous avons écrit et vérifient que son comportement est correct.

Modifions le fichier _src/lib.rs_. Son contenu devrait ressembler à l'extrait de code ci-dessous.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

##### Exemple d'un module de test et d'une fonction générée automatiquement par cargo new

Pour l'instant, ignorons les deux premières lignes et concentrons-nous sur la fonction pour voir comment elle fonctionne. Notez l'annotation `#[test]` avant la ligne `fn` : cet attribut indique qu'il s'agit d'une fonction de test, de sorte que le test runner sache traiter cette fonction comme un test. Nous pourrions également avoir des fonctions non-test dans le module `tests` pour aider à mettre en place des scénarios courants ou effectuer des opérations courantes, donc nous devons indiquer quelles fonctions sont des tests en utilisant l'attribut `#[test]`.

Le corps de la fonction utilise la macro `assert_eq!` pour affirmer que 2 + 2 égale 4. Cette assertion sert d'exemple du format pour un test typique. Exécutons-le pour voir que ce test réussit.

Cliquez avec le bouton droit sur la tâche 'Comment écrire des tests' et choisissez **Ouvrir dans le terminal** et exécutez la commande `cargo test`. Vous verrez une sortie similaire à celle indiquée ci-dessous.

```text
$ cargo test
  Compiling how_to_write_tests v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Exemple d'une sortie de l'exécution du test généré automatiquement

Cargo a compilé et exécuté le test. Après les lignes `Compiling`, `Finished`, et `Running`, se trouve la ligne `running 1 test`. La ligne suivante montre le nom de la fonction de test générée, appelée `it_works`, et le résultat de l'exécution de ce test, `ok`. Le résumé global de l'exécution des tests apparaît ensuite. Le texte `test result: ok.` signifie que tous les tests ont réussi, et la partie qui indique `1 passed; 0 failed` totalise le nombre de tests réussis ou échoués.

Comme nous n'avons pas de tests que nous avons marqués comme ignorés, le résumé affiche `0 ignored`. Nous n'avons également pas filtré les tests exécutés, donc la fin du résumé montre `0 filtered out`. Nous parlerons de l'ignorance et du filtrage des tests dans la section "Exécution des tests".

La statistique `0 measured` est pour les tests de performance. Les tests de performance sont, au moment de cet écrit, uniquement disponibles dans Rust nightly. Consultez [la documentation sur les tests de performance](https://doc.rust-lang.org/unstable-book/library-features/test.html) pour en savoir plus.

La partie suivante de la sortie de test, qui commence par `Doc-tests how_to_write_tests`, concerne les résultats de tous les tests de documentation. Nous n'avons pas encore de tests de documentation, mais Rust peut compiler tous les exemples de code qui apparaissent dans notre documentation API. Cette fonctionnalité nous aide à garder nos docs et notre code en synchronisation ! Nous discuterons de la manière d'écrire des tests de documentation dans la section ["Commentaires de documentation en tant que tests"](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests) du chapitre 14 du livre Rust. Pour l'instant, nous ignorerons la sortie `Doc-tests`.

Changeons le nom de notre test pour voir comment cela modifie la sortie du test. Changez la fonction `it_works` en un autre nom, tel que `exploration`, comme suit :

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
    }
```

Puis exécutez à nouveau `cargo test`. La sortie montre maintenant `exploration` au lieu de `it_works` :

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Ajoutons un autre test, mais cette fois nous créerons un test qui échoue ! Les tests échouent lorsque quelque chose dans la fonction de test déclenche une panique. Chaque test est exécuté dans un nouveau thread, et lorsque le thread principal constate qu'un thread de test est mort, le test est marqué comme échoué. Nous avons parlé de la manière la plus simple de provoquer une panique au chapitre 9, qui consiste à appeler la macro `panic!`. Entrez le nouveau test, `another`, pour que votre fichier _src/lib.rs_ ressemble à l'extrait de code ci-dessous.

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn another() {
            panic!("Make this test fail");
        }
    }
```

##### Exemple d'ajout d'un deuxième test qui échouera car nous appelons la macro panic!

Exécutez à nouveau les tests en utilisant `cargo test`. La sortie devrait ressembler à celle ci-dessous, qui montre que notre test `exploration` a réussi et `another` a échoué.

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', Writing Automated Tests/Tests/How to Write Tests/src/lib.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

##### Exemple des résultats de test lorsqu'un test réussit et qu'un test échoue

Au lieu de `ok`, la ligne `test tests::another` montre `FAILED`. Deux nouvelles sections apparaissent entre les résultats individuels et le résumé : la première section affiche la raison détaillée de chaque échec de test. Dans ce cas, `another` a échoué car il `panicked at "Make this test fail"`, ce qui s'est produit à la ligne 10 du fichier _src/lib.rs_. La section suivante liste simplement les noms de tous les tests échoués, ce qui est utile lorsqu'il y a beaucoup de tests et beaucoup de détails sur les échecs de test. Nous pouvons utiliser le nom d'un test échoué pour exécuter seulement ce test afin de le déboguer plus facilement ; vous pouvez en savoir plus sur les façons d'exécuter les tests dans la section "Exécution des tests".

La ligne de résumé s'affiche à la fin : globalement, notre résultat de test est `FAILED`. Nous avons eu un test réussi et un test échoué.

Maintenant que vous avez vu à quoi ressemblent les résultats de test dans différents scénarios, examinons quelques macros autres que `panic!` qui sont utiles dans les tests.