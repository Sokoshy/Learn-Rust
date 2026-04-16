### Capturer des références ou transférer la propriété

Les fermetures peuvent capturer des valeurs de leur environnement de trois manières, ce qui correspond directement aux trois manières dont une fonction peut prendre un paramètre : en empruntant de manière immuable, en empruntant de manière mutable et en prenant la propriété. La fermeture décidera laquelle de ces méthodes utiliser en fonction de ce que le corps de la fonction fait avec les valeurs capturées.

La Liste suivante définit une fermeture qui capture un emprunt immuable au vecteur nommé `list` car elle n'a besoin que d'un emprunt immuable pour imprimer la valeur. Cet exemple illustre également qu'une variable peut être liée à une définition de fermeture, et la fermeture peut être appelée plus tard en utilisant le nom de la variable et des parenthèses comme si le nom de la variable était un nom de fonction :

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Avant de définir la fermeture : {:?}", list);

    let only_borrows = || println!("Depuis la fermeture : {:?}", list);

    println!("Avant d'appeler la fermeture : {:?}", list);
    only_borrows();
    println!("Après avoir appelé la fermeture : {:?}", list);
}
```

##### Exemple de définition et d'appel d'une fermeture qui capture un emprunt immuable

La `list` est toujours accessible par le code avant la définition de la fermeture, après la définition de la fermeture mais avant que la fermeture soit appelée, et après que la fermeture soit appelée car nous pouvons avoir plusieurs emprunts immuables de `list` en même temps. Ce code se compile, s'exécute et affiche :

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Avant de définir la fermeture : [1, 2, 3]
Avant d'appeler la fermeture : [1, 2, 3]
Depuis la fermeture : [1, 2, 3]
Après avoir appelé la fermeture : [1, 2, 3]
```

La Liste suivante modifie la définition de la fermeture pour nécessiter un emprunt mutable car le corps de la fermeture ajoute un élément au vecteur `list` :

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Avant de définir la fermeture : {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("Après avoir appelé la fermeture : {:?}", list);
}
```

##### Exemple de définition et d'appel d'une fermeture qui capture un emprunt mutable

Ce code se compile, s'exécute et affiche :

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Avant de définir la fermeture : [1, 2, 3]
Après avoir appelé la fermeture : [1, 2, 3, 7]
```

Notez qu'il n'y a plus de `println!` entre la définition et l'appel de la fermeture `borrows_mutably` : lorsque `borrows_mutably` est définie, elle capture une référence mutable à `list`. Après que la fermeture est appelée, parce que nous n'utilisons plus la fermeture après ce point, l'emprunt mutable se termine. Entre la définition de la fermeture et l'appel de la fermeture, un emprunt immuable pour l'impression n'est pas autorisé car aucun autre emprunt n'est autorisé lorsqu'il y a un emprunt mutable. Essayez d'ajouter un `println!` à cet endroit pour voir quel message d'erreur vous obtenez !

Si vous voulez forcer la fermeture à prendre la propriété des valeurs qu'elle utilise dans l'environnement même si le corps de la fermeture n'a pas strictement besoin de la propriété, vous pouvez utiliser le mot-clé `move` avant la liste des paramètres. Cette technique est surtout utile lorsque l'on passe une fermeture à un nouveau fil d'exécution pour déplacer les données afin qu'elles soient détenues par le nouveau fil. Nous aurons plus d'exemples de fermetures `move` dans le chapitre 16 lorsque nous parlerons de la concurrence.