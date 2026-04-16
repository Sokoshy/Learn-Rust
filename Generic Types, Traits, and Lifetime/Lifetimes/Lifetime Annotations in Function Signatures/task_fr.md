### Annotations de durée de vie dans les signatures de fonction

Examinons maintenant les annotations de durée de vie dans le contexte de la fonction `longest`. Comme avec les paramètres de type génériques, nous devons déclarer des paramètres de durée de vie génériques entre crochets angulaires entre le nom de la fonction et la liste des paramètres. La contrainte que nous voulons exprimer dans cette signature est que toutes les références dans les paramètres et la valeur de retour doivent avoir la même durée de vie. Nous nommerons la durée de vie `'a` et l'ajouterons ensuite à chaque référence, comme illustré dans l'extrait de code ci-dessous.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### La définition de la fonction `longest` spécifiant que toutes les références dans la signature doivent avoir la même durée de vie `'a`

Ce code devrait se compiler et produire le résultat souhaité lorsque nous l'utilisons avec la fonction `main` dans l'exemple où il appelle la fonction `longest` pour trouver la plus longue des deux tranches de chaîne.

La signature de la fonction indique maintenant à Rust que pour une certaine durée de vie `'a`, la fonction prend deux paramètres, qui sont des tranches de chaîne vivant au moins aussi longtemps que la durée de vie `'a`. La signature de la fonction indique également à Rust que la tranche de chaîne retournée par la fonction vivra au moins aussi longtemps que la durée de vie `'a`. En pratique, cela signifie que la durée de vie de la référence retournée par la fonction `longest` est la même que celle de la plus petite des durées de vie des références passées en paramètres. Ces contraintes sont ce que nous voulons que Rust impose. Rappelez-vous, lorsque nous spécifions les paramètres de durée de vie dans cette signature de fonction, nous ne modifions pas les durées de vie des valeurs passées ou retournées. Nous spécifions plutôt que le vérificateur d'emprunt doit rejeter toutes les valeurs qui ne respectent pas ces contraintes. Notez que la fonction `longest` n'a pas besoin de savoir exactement combien de temps `x` et `y` vivront, seulement qu'une portée peut être substituée pour `'a` qui satisfera cette signature.

Lors de l'annotation des durées de vie dans les fonctions, les annotations vont dans la signature de la fonction, pas dans le corps de la fonction. Rust peut analyser le code à l'intérieur de la fonction sans aide. Cependant, lorsqu'une fonction a des références vers ou depuis un code en dehors de cette fonction, il devient presque impossible pour Rust de déterminer les durées de vie des paramètres ou des valeurs de retour par lui-même. Les durées de vie pourraient être différentes à chaque fois que la fonction est appelée. C'est pourquoi nous devons annoter les durées de vie manuellement.

Quand nous passons des références concrètes à `longest`, la durée de vie concrète qui est substituée à `'a` est la partie de la portée de `x` qui chevauche la portée de `y`. En d'autres termes, la durée de vie générique `'a` obtiendra la durée de vie concrète égale à la plus petite des durées de vie de `x` et `y`. Puisque nous avons annoté la référence retournée avec le même paramètre de durée de vie `'a`, la référence retournée sera également valide pour la durée de la plus petite des durées de vie de `x` et `y`.

Voyons comment les annotations de durée de vie limitent la fonction `longest` en passant des références ayant des durées de vie concrètes différentes. Le code ci-dessous est un exemple simple.

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

#### Utilisation de la fonction `longest` avec des références à des valeurs `String` ayant différentes durées de vie concrètes

Dans cet exemple, `string1` est valide jusqu'à la fin de la portée extérieure, `string2` est valide jusqu'à la fin de la portée intérieure, et `result` fait référence à quelque chose qui est valide jusqu'à la fin de la portée intérieure. Exécutez ce code et vous verrez que le vérificateur d'emprunt approuve ce code; il se compilera et affichera `The longest string is long string is long`.