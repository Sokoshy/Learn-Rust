## Parcourir un tableau avec for

Vous pourriez utiliser la structure `while` pour parcourir les éléments d'une collection, telle qu'un tableau. Par exemple, examinons le code suivant :

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("la valeur est : {}", a[index]);

        index += 1;
    }
}
```
##### Exemple de parcours de chaque élément d'une collection à l'aide d'une boucle while

Ici, le code parcourt les éléments du tableau en comptant. Il commence à l'index `0`, puis boucle jusqu'à atteindre le dernier index du tableau (c'est-à-dire lorsque `index < 5` n'est plus vrai). L'exécution de ce code affichera chaque élément du tableau :

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
la valeur est : 10
la valeur est : 20
la valeur est : 30
la valeur est : 40
la valeur est : 50
```

Les cinq valeurs du tableau apparaissent dans le terminal, comme prévu. Bien que `index` atteindra une valeur de `5` à un moment donné, la boucle s'arrête avant d'essayer de récupérer une sixième valeur du tableau.

Cependant, cette approche est sujette aux erreurs ; nous pourrions provoquer un panic du programme si la longueur de l'index est incorrecte. C'est également lent car le compilateur ajoute du code d'exécution pour effectuer la vérification conditionnelle à chaque élément à chaque itération de la boucle.

Comme alternative plus concise, vous pouvez utiliser une boucle `for` et exécuter du code pour chaque élément d'une collection. Une boucle for ressemble à ceci :

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("la valeur est : {}", element);
    }
}
```
##### Exemple de parcours de chaque élément d'une collection à l'aide d'une boucle for

Lorsque nous exécutons ce code, nous verrons la même sortie que dans l'extrait de code précédent. Plus important encore, nous avons maintenant accru la sécurité du code et éliminé les risques de bugs pouvant résulter d'un dépassement de la fin du tableau ou de ne pas parcourir suffisamment le tableau et manquer certains éléments.

Par exemple, dans l'extrait de code précédent, si vous modifiez la définition du tableau `a` pour qu'il ait quatre éléments, mais oubliez de mettre à jour la condition à `while index < 4`, le code provoquerait un panic. En utilisant la boucle `for`, vous n'auriez pas besoin de vous souvenir de modifier toute autre partie du code si vous changez le nombre de valeurs dans le tableau.