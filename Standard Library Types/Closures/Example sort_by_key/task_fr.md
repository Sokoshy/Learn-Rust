### Exemple : `sort_by_key`

Voyons maintenant la méthode de la bibliothèque standard `sort_by_key` définie sur les tranches, pour voir en quoi elle diffère. Elle prend une fermeture qui implémente `FnMut`. La fermeture reçoit un argument, une référence à l'élément actuel de la tranche en cours de traitement, et renvoie une valeur de type `K` qui peut être ordonnée. Cette fonction est utile lorsque vous souhaitez trier une tranche par un attribut particulier de chaque élément. Dans l'exemple suivant, nous avons une liste d'instances `Rectangle` et nous utilisons `sort_by_key` pour les trier par leur attribut `width`, de bas en haut :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

##### Exemple d'utilisation de `sort_by_key` et d'une fermeture pour trier une liste d'instances de `Rectangle` par leur valeur `width`

Ce code affiche :

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

La raison pour laquelle `sort_by_key` est défini pour prendre une fermeture `FnMut` est qu'il appelle la fermeture plusieurs fois : une fois pour chaque élément de la tranche. La fermeture `|r| r.width` ne capture, ne modifie ni ne déplace rien de son environnement, donc elle répond aux exigences du contrat de trait.

En revanche, le prochain exemple montre une fermeture qui n'implémente que `FnOnce` car elle déplace une valeur hors de l'environnement. Le compilateur ne nous permettra pas d'utiliser cette fermeture avec `sort_by_key` :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
```

##### Exemple de tentative d'utilisation d'une fermeture `FnOnce` avec `sort_by_key`

C'est une manière artificielle et complexe (qui ne fonctionne pas) de tenter de compter le nombre de fois où `sort_by_key` est appelé lors du tri de `list`. Ce code essaie de faire ce comptage en poussant `value`, un `String` de l'environnement de la fermeture, dans le vecteur `sort_operations`. La fermeture capture `value` puis déplace `value` hors de la fermeture en transférant la propriété de `value` au vecteur `sort_operations`. Cette fermeture peut être appelée une fois ; la tenter une deuxième fois ne fonctionnerait pas car `value` ne serait plus dans l'environnement pour être poussé à nouveau dans `sort_operations` ! C'est pourquoi cette fermeture n'implémente que `FnOnce`. Lorsque nous essayons de compiler ce code, nous obtenons cette erreur indiquant que `value` ne peut pas être déplacé hors de la fermeture car elle doit implémenter `FnMut` :

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:30
   |
24 |       let value = String::from("by key called");
   |           ----- captured outer variable
25 | 
26 |       list.sort_by_key(|r| {
   |  ______________________-
27 | |         sort_operations.push(value);
   | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
28 | |         r.width
29 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` due to previous error
```

L'erreur pointe vers la ligne dans le corps de la fermeture qui déplace `value` hors de l'environnement. Pour corriger cela, nous devons changer le corps de la fermeture pour qu'il ne déplace pas les valeurs hors de l'environnement. Si nous sommes intéressés par le nombre de fois où `sort_by_key` est appelé, garder un compteur dans l'environnement et incrémenter sa valeur dans le corps de la fermeture est un moyen plus simple de le calculer. La fermeture dans l'exemple suivant fonctionne avec `sort_by_key` parce qu'elle ne fait que capturer une référence mutable vers le compteur `num_sort_operations` et peut donc être appelée plus d'une fois :

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
```

##### Exemple d'utilisation d'une fermeture `FnMut` avec `sort_by_key` est autorisé

Les traits `Fn` sont importants lors de la définition ou de l'utilisation de fonctions ou de types qui font usage de fermetures. La section suivante aborde les itérateurs, et de nombreuses méthodes d'itérateurs prennent des arguments de fermeture. Gardez ces détails sur les fermetures à l'esprit alors que nous explorons les itérateurs !