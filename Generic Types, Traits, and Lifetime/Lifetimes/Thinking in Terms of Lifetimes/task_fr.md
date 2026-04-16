### Penser en termes de durées de vie

La manière dont vous devez spécifier les paramètres de durée de vie dépend de ce que fait votre fonction. Par exemple, si nous modifions l'implémentation de la fonction `longest` pour qu'elle retourne toujours le premier paramètre au lieu de la sous-chaîne la plus longue, nous n'aurions pas besoin de spécifier une durée de vie pour le paramètre `y`. Le code suivant se compilera :

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

Dans cet exemple, nous avons spécifié un paramètre de durée de vie `'a` pour le paramètre `x` et le type de retour, mais pas pour le paramètre `y`, car la durée de vie de `y` n’a pas de relation avec la durée de vie de `x` ou avec la valeur de retour.

Lorsqu'on retourne une référence à partir d'une fonction, le paramètre de durée de vie pour le type de retour doit correspondre au paramètre de durée de vie de l'un des paramètres. Si la référence retournée ne se réfère *pas* à l’un des paramètres, elle doit se référer à une valeur créée dans cette fonction, ce qui serait une référence pendante car la valeur sortira du champ d'application à la fin de la fonction. Considérez cette tentative d'implémentation de la fonction `longest` qui ne se compilera pas :

```rust,ignore,does_not_compile
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Ici, bien que nous ayons spécifié un paramètre de durée de vie `'a` pour le type de retour, cette implémentation échouera lors de la compilation car la durée de vie de la valeur de retour n'est pas du tout liée à la durée de vie des paramètres. Voici le message d'erreur que nous obtenons :

```console
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here
```

Le problème est que `result` sort du champ d'application et est nettoyé à la fin de la fonction `longest`. Nous essayons également de retourner une référence à `result` à partir de la fonction. Il n'y a aucun moyen de spécifier des paramètres de durée de vie qui changeraient la référence pendante, et Rust ne nous permettra pas de créer une référence pendante. Dans ce cas, la meilleure solution serait de retourner un type de données possédé plutôt qu'une référence, afin que la fonction appelante soit alors responsable du nettoyage de la valeur.

En fin de compte, la syntaxe des durées de vie concerne la connexion des durées de vie des divers paramètres et valeurs de retour des fonctions. Une fois qu'elles sont connectées, Rust dispose de suffisamment d'informations pour permettre des opérations sûres en mémoire et interdire des opérations qui créeraient des pointeurs pendants ou violeraient autrement la sécurité de la mémoire.