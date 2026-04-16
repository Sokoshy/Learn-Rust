### Fournir de nouveaux noms avec le mot-clé `as`

Il existe une autre solution au problème d'importer deux types portant le même nom dans le même espace de noms avec `use` : après le chemin, nous pouvons spécifier `as` et un nouveau nom local, ou alias, pour le type. L'exemple suivant montre une autre façon d'écrire le code pour l'importation en renommant l'un des deux types `Result` en utilisant `as`.

```rust
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
    }

    fn function2() -> IoResult<()> {
    }
```

##### Renommer un type lors de son importation dans l'espace de noms avec le mot-clé as

Dans la deuxième déclaration `use`, nous avons choisi le nouveau nom `IoResult` pour le type `std::io::Result`, qui ne sera pas en conflit avec le `Result` de `std::fmt` que nous avons également importé dans l'espace de noms. Les deux exemples ci-dessus sont considérés comme idiomatiques, donc le choix vous appartient !