## Contrôler l'exécution des tests

Tout comme `cargo run` compile votre code puis exécute le binaire résultant, `cargo test` compile votre code en mode test et exécute le binaire de test résultant. Vous pouvez spécifier des options en ligne de commande pour modifier le comportement par défaut de `cargo test`. Par exemple, le comportement par défaut du binaire produit par `cargo test` est d'exécuter tous les tests en parallèle et de capturer la sortie générée pendant les tests, empêchant ainsi la sortie d'être affichée et facilitant la lecture de la sortie liée aux résultats des tests.

Certaines options de ligne de commande sont destinées à `cargo test`, et d'autres au binaire de test résultant. Pour séparer ces deux types d'arguments, vous listez les arguments destinés à `cargo test` suivis du séparateur `--` et puis ceux destinés au binaire de test. L'exécution de `cargo test --help` affiche les options que vous pouvez utiliser avec `cargo test`, et l'exécution de `cargo test -- --help` affiche les options que vous pouvez utiliser après le séparateur `--`.

### Exécution des tests en parallèle ou consécutivement

Lorsque vous exécutez plusieurs tests, par défaut, ils s'exécutent en parallèle en utilisant des threads. Cela signifie que les tests se termineront plus rapidement, vous permettant d'avoir un retour plus rapide sur le bon fonctionnement de votre code. Puisque les tests s'exécutent en même temps, assurez-vous que vos tests ne dépendent pas les uns des autres ou d'un état partagé, y compris un environnement partagé, comme le répertoire de travail actuel ou les variables d'environnement.

Par exemple, disons que chacun de vos tests exécute un code qui crée un fichier sur le disque nommé *test-output.txt* et écrit des données dans ce fichier. Ensuite, chaque test lit les données dans ce fichier et affirme que ce fichier contient une valeur particulière, qui est différente dans chaque test. Étant donné que les tests s'exécutent en même temps, un test pourrait écraser le fichier entre le moment où un autre test écrit et lit le fichier. Le second test échouerait alors, non pas parce que le code est incorrect, mais parce que les tests se sont interférés entre eux lors de leur exécution en parallèle. Une solution consiste à faire en sorte que chaque test écrive dans un fichier différent ; une autre solution est d'exécuter les tests un par un.

Si vous ne souhaitez pas exécuter les tests en parallèle ou si vous voulez un contrôle plus fin sur le nombre de threads utilisés, vous pouvez envoyer l'option `--test-threads` et le nombre de threads que vous souhaitez utiliser au binaire de test. Jetez un œil à l'exemple suivant :

```console
$ cargo test -- --test-threads=1
```

Nous définissons le nombre de threads de test à `1`, indiquant au programme de ne pas utiliser de parallélisme. Exécuter les tests avec un thread prendra plus de temps que de les exécuter en parallèle, mais les tests ne se perturberont pas s'ils partagent un état.

### Affichage de la sortie des fonctions

Par défaut, si un test réussit, la bibliothèque de tests de Rust capture tout ce qui est imprimé sur la sortie standard. Par exemple, si nous appelons `println!` dans un test et que le test réussit, nous ne verrons pas la sortie de `println!` dans le terminal; nous ne verrons que la ligne indiquant que le test a réussi. Si un test échoue, nous verrons ce qui a été imprimé sur la sortie standard avec le reste du message d'échec.

À titre d'exemple, le code ci-dessous a une fonction simple qui imprime la valeur de son paramètre et renvoie 10, ainsi qu'un test qui réussit et un autre qui échoue.

```rust,panics
fn prints_and_returns_10(a: i32) -> i32 {
    println!("J'ai obtenu la valeur {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

##### Tests pour une fonction qui appelle `println!`

Lorsque nous exécutons ces tests avec `cargo test`, nous verrons la sortie suivante :

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
J'ai obtenu la valeur 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Notez qu'à aucun endroit dans cette sortie nous ne voyons `J'ai obtenu la valeur 4`, qui est ce qui est imprimé lorsque le test qui réussit est exécuté. Cette sortie a été capturée. La sortie du test qui a échoué, `J'ai obtenu la valeur 8`, apparaît dans la section du résumé des tests, qui montre également la cause de l'échec du test.

Si nous voulons voir les valeurs imprimées pour les tests réussis aussi, nous pouvons dire à Rust de montrer également la sortie des tests réussis à la fin avec `--show-output`.

```text
$ cargo test -- --show-output
```

Lorsque nous exécutons à nouveau les tests dans "Tests pour une fonction qui appelle `println!`" avec l'option `--show-output`, nous voyons la sortie suivante :

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
J'ai obtenu la valeur 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
J'ai obtenu la valeur 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

### Exécuter un sous-ensemble de tests par nom

Parfois, exécuter une suite de tests complète peut prendre beaucoup de temps. Si vous travaillez sur du code dans un domaine particulier, vous voudrez peut-être n'exécuter que les tests relatifs à ce code. Vous pouvez choisir quels tests exécuter en passant à `cargo test` le nom ou les noms des tests que vous souhaitez exécuter comme argument.

Pour démontrer comment exécuter un sous-ensemble de tests, nous allons créer trois tests pour notre fonction `add_two`, comme illustré ci-dessous, et choisir lesquels exécuter.

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

##### Trois tests avec trois noms différents

Si nous exécutons les tests sans passer d'arguments, comme nous l'avons vu précédemment, tous les tests seront exécutés en parallèle :

```text
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### Exécution de tests uniques

Nous pouvons transmettre le nom de toute fonction de test à `cargo test` pour n'exécuter que ce test, par exemple, `cargo test one_hundred` :

```text
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

Seul le test avec le nom `one_hundred` s'est exécuté; les deux autres tests ne correspondaient pas à ce nom. La sortie des tests nous informe que nous avions plus de tests que ce que cette commande a exécuté en affichant `2 filtered out` à la fin de la ligne du résumé.

Nous ne pouvons pas spécifier les noms de plusieurs tests de cette manière; seule la première valeur donnée à `cargo test` sera utilisée. Mais il y a un moyen d'exécuter plusieurs tests.

#### Filtrer pour exécuter plusieurs tests

Nous pouvons spécifier une partie du nom d'un test, et tout test dont le nom correspondra à cette valeur sera exécuté. Par exemple, comme deux de nos tests ont des noms contenant `add`, nous pouvons exécuter ces deux en lançant `cargo test add` :

```text
running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

Cette commande a exécuté tous les tests avec `add` dans le nom et a filtré le test nommé `one_hundred`. Notez également que le module dans lequel un test apparaît devient une partie du nom du test, nous pouvons donc exécuter tous les tests d'un module en filtrant sur le nom du module.

### Ignorer certains tests sauf sur demande spécifique

Parfois, quelques tests spécifiques peuvent être très longs à exécuter, vous pourriez donc vouloir les exclure lors de la plupart des exécutions de `cargo test`. Plutôt que de lister comme arguments tous les tests que vous voulez exécuter, vous pouvez annoter les tests longs avec l'attribut `ignore` pour les exclure, comme illustré ici :

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code qui prend une heure à s'exécuter
}
```

Après `#[test]`, nous ajoutons la ligne `#[ignore]` au test que nous voulons exclure. Maintenant, lorsque nous exécutons nos tests, `it_works` s'exécute, mais `expensive_test` non :

```text
running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

La fonction `expensive_test` est listée comme `ignored`. Si nous voulons exécuter uniquement les tests ignorés, nous pouvons utiliser `cargo test -- --ignored` :

```text
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

En contrôlant quels tests s'exécutent, vous pouvez vous assurer que les résultats de votre `cargo test` seront rapides. Lorsque vous en êtes à un point où il est logique de vérifier les résultats des tests `ignored` et que vous avez du temps pour attendre les résultats, vous pouvez lancer `cargo test -- --ignored` à la place.

_Vous pouvez vous référer au chapitre suivant du livre de programmation Rust : [Contrôler l'exécution des tests](https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html)_