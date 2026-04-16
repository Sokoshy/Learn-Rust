## Erreurs avancées 2

Cet exercice démontre quelques approches utiles pour les types d'erreurs personnalisées, notamment pour que d'autres codes puissent consommer le type d'erreur personnalisé de manière plus utile.

Faites en sorte que cela compile, et que les tests réussissent !
Consultez les indices si vous êtes bloqué.

Étapes :
1. Assurez-vous que `ParseClimateError` est adapté à la propagation d'erreurs dans `main()`.
2. Complétez l'implémentation partielle de `From` pour `ParseClimateError`.
3. Gérez les cas d'erreur manquants dans l'implémentation de `FromStr` pour `Climate` en utilisant les directives de parsing ci-dessous.
4. Complétez l'implémentation partielle de `Display` pour `ParseClimateError`.

Parseur pour `Climate` :
1. Divisez la chaîne d'entrée en 3 champs : `city`, `year`, `temp`.
2. Retournez une erreur si la chaîne est vide ou si elle a un nombre de champs incorrect.
3. Retournez une erreur si le nom de la ville est vide.
4. Analysez l'année en tant que `u32` et retournez une erreur si cela échoue.
5. Analysez la température en tant que `f32` et retournez une erreur si cela échoue.
6. Retournez une valeur `Ok` contenant la valeur `Climate` complétée.

Ajoutez ou complétez les étapes manquantes.
N'oubliez pas de lire les commentaires dans le code.

<div class="hint">

Regardons le fichier `main.rs`. Le type de résultat de la fonction `main` est `Result<(), Box<dyn Error>>`. Cela signifie que nous pouvons retourner soit `()`, soit *n'importe quelle erreur*. Plus loin dans la fonction `main`, nous utilisons la propagation d'erreurs :

```rust
  "Hong Kong,1999,25.7".parse::<Climate>()?
```

Ce code propage l'erreur personnalisée `ParseClimateError`. Pour la rendre adaptée au passage comme *n'importe quelle erreur*, nous devons en faire une `Error` comme suit :
```rust
impl Error for ParseClimateError {}
```

Le `Error` (`std::error::Error`) est un trait qui représente les attentes fondamentales pour les valeurs d'erreur. Nous discuterons des traits plus en détail [plus tard dans ce cours](course://Generic%20Types,%20Traits,%20and Lifetime/Traits/Traits).
</div>

<div class="hint">
Il n'est pas nécessaire d'implémenter des méthodes à l'intérieur de l'implémentation de <code>Error</code>. (Certaines méthodes ont des implémentations fournies par défaut.)
</div>

<div class="hint">Consultez les tests pour déterminer quelles variantes d'erreur (et quel texte de message d'erreur) produire pour certaines conditions d'erreur.</div>

<div class="hint">
Vous pourriez trouver ces pages utiles comme références :
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html">1</a>,
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">2</a>, 
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html">3</a>.
</div>

**Défi** : Il y a un test qui est marqué `#[ignore]`. Pouvez-vous fournir le code manquant qui le fera réussir ? Vous pourriez vouloir consulter la documentation de la bibliothèque standard pour un certain trait pour plus d'indices.