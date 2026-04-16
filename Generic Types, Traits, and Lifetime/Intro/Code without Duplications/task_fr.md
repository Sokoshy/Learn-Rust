## Code sans duplications

Dans l'extrait de code suivant, nous avons extrait le code qui trouve le plus grand nombre dans une fonction nommée `largest`. Contrairement au code dans le premier exemple de cette section, qui peut trouver le plus grand nombre uniquement dans une liste particulière, ce programme peut trouver le plus grand nombre dans deux listes différentes.

```rust
fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("Le plus grand nombre est {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("Le plus grand nombre est {}", result);
}
```

#### Code abstrait pour trouver le plus grand nombre dans deux listes

La fonction `largest` a un paramètre appelé `list`, qui représente n'importe quelle tranche concrète de valeurs `i32` que nous pourrions passer à la fonction. En conséquence, lorsque nous appelons la fonction, le code s'exécute sur les valeurs spécifiques que nous transmettons.

En résumé, voici les étapes que nous avons suivies pour modifier le code du deuxième au troisième exemple :

1. Identifier le code dupliqué.
2. Extraire le code dupliqué dans le corps de la fonction et spécifier les entrées et les valeurs de retour de ce code dans la signature de la fonction.
3. Mettre à jour les deux instances de code dupliqué pour appeler la fonction à la place.

Ensuite, nous utiliserons ces mêmes étapes avec des générics pour réduire la duplication de code de différentes manières. De la même manière que le corps de la fonction peut opérer sur une `list` abstraite au lieu de valeurs spécifiques, les générics permettent au code de fonctionner sur des types abstraits.

Par exemple, disons que nous avions deux fonctions : une qui trouve l'élément le plus grand dans une tranche de valeurs `i32` et une qui trouve l'élément le plus grand dans une tranche de valeurs `char`. Comment éliminerions-nous cette duplication ? Découvrons-le !