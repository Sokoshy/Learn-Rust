### Correction de la fonction `largest` avec des limites de traits

Maintenant que vous savez comment spécifier le comportement que vous souhaitez utiliser en utilisant les limites du paramètre de type générique, revenons à la [définition de la fonction `largest`](course://Generic+Types,+Traits,+and+Lifetime/Generic+Data+Types/In+Function+Definitions) qui utilise un paramètre de type générique ! La dernière fois que nous avons essayé d'exécuter ce code, nous avons reçu cette erreur :

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

Dans le corps de `largest`, nous voulions comparer deux valeurs de type `T` en utilisant l'opérateur supérieur à (`>`). Étant donné que cet opérateur est défini comme une méthode par défaut dans le trait de la bibliothèque standard `std::cmp::PartialOrd`, nous devons spécifier `PartialOrd` dans les limites de trait pour `T` afin que la fonction `largest` puisse fonctionner sur des tranches de n'importe quel type que nous pouvons comparer. Nous n'avons pas besoin d'importer `PartialOrd` car il fait partie de la préface. Modifiez la signature de `largest` pour ressembler à ceci :

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

Cette fois, lorsque nous compilons le code, nous obtenons un autre ensemble d'erreurs :

```console
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

Certaines erreurs ont des explications détaillées : E0507, E0508.
Pour plus d'informations sur une erreur, essayez `rustc --explain E0507`.
error: could not compile `chapter10` due to 2 previous errors
```

La ligne clé dans cette erreur est `cannot move out of type [T], a non-copy slice`. Avec nos versions non génériques de la fonction `largest`, nous essayions uniquement de trouver le plus grand `i32` ou `char`. Comme discuté dans la sous-section “Données uniquement sur la pile : Copier” dans la section [Cloner et Copier](course://Understanding Ownership/What is ownership/Clone and Copy), les types tels que `i32` et `char` qui ont une taille connue peuvent être stockés sur la pile, de sorte qu'ils implémentent le trait `Copy`. Toutefois, lorsque nous avons rendu la fonction `largest` générique, il est devenu possible pour le paramètre `list` de contenir des types qui n'implémentent pas le trait `Copy`. Par conséquent, nous ne pourrions pas déplacer la valeur de `list[0]` dans la variable `largest`, ce qui entraîne cette erreur.

Pour appeler ce code uniquement avec les types qui implémentent le trait `Copy`, nous pouvons ajouter `Copy` aux limites de trait de `T` ! Le code ci-dessous montre le code complet d'une fonction `largest` générique qui se compilera tant que les types des valeurs dans la tranche que nous passons à la fonction implémentent les traits `PartialOrd` *et* `Copy`, comme le font `i32` et `char`.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

#### Une définition fonctionnelle de la fonction `largest` qui fonctionne sur n'importe quel type générique implémentant les traits `PartialOrd` et `Copy`

Si nous ne voulons pas restreindre la fonction `largest` aux types qui implémentent le trait `Copy`, nous pourrions spécifier que `T` a la limite de trait `Clone` au lieu de `Copy`. Ensuite, nous pourrions cloner chaque valeur dans la tranche lorsque nous voulons que la fonction `largest` ait la propriété. Utiliser la fonction `clone` signifie que nous effectuons potentiellement plus d'allocations sur le tas dans le cas de types qui possèdent des données sur le tas comme `String`, et les allocations sur le tas peuvent être lentes si nous manipulons de grandes quantités de données.

Une autre manière d'implémenter `largest` est que la fonction renvoie une référence à une valeur `T` dans la tranche. Si nous changeons le type de retour en `&T` au lieu de `T`, modifiant ainsi le corps de la fonction pour renvoyer une référence, nous n'aurions pas besoin des limites de traits `Clone` ou `Copy` et nous pourrions éviter les allocations sur le tas. Essayez de mettre en œuvre ces solutions alternatives par vous-même !