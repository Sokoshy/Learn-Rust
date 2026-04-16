## Paramètres de type génériques, limites de traits et durées de vie ensemble

Examinons brièvement la syntaxe consistant à spécifier des paramètres de type génériques, des limites de traits, et des durées de vie dans une seule fonction !

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Annonce ! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Ceci est la fonction `longest` vue précédemment qui renvoie la plus longue des deux tranches de chaîne de caractères. Mais maintenant, elle possède un paramètre supplémentaire nommé `ann` de type générique `T`, qui peut être rempli par n'importe quel type implémentant le trait `Display` tel que spécifié par la clause `where`. Ce paramètre supplémentaire sera imprimé avant que la fonction ne compare les longueurs des tranches de chaîne de caractères, c'est pourquoi la limite de trait `Display` est nécessaire. Étant donné que les durées de vie sont un type de génériques, les déclarations du paramètre de durée de vie `'a` et du paramètre de type générique `T` se placent dans la même liste entre les crochets angulaires après le nom de la fonction.