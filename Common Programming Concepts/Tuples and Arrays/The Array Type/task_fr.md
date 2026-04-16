## Le type tableau

Une autre façon d'avoir une collection de plusieurs valeurs est avec un _tableau_. Contrairement à un tuple, chaque élément d'un tableau doit être du même type. Les tableaux en Rust diffèrent de ceux d'autres langages parce qu'en Rust, les tableaux ont une longueur fixe, comme les tuples.

En Rust, les valeurs entrant dans un tableau sont écrites sous forme de liste séparée par des virgules à l'intérieur de crochets :

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Les tableaux sont utiles lorsque vous souhaitez que vos données soient allouées sur la pile plutôt que sur le tas (nous discuterons de la pile et du tas plus en détail au chapitre 4) ou lorsque vous souhaitez vous assurer d'avoir toujours un nombre fixe d'éléments. Cependant, un tableau n'est pas aussi flexible que le type `vecteur`. Un vecteur est un type de collection similaire fourni par la bibliothèque standard qui est autorisé à croître ou diminuer en taille. Si vous n'êtes pas sûr de devoir utiliser un tableau ou un vecteur, vous devriez probablement utiliser un vecteur. Le chapitre 8 discute des vecteurs plus en détail.

Un exemple où vous voudrez peut-être utiliser un tableau plutôt qu'un vecteur est dans un programme qui doit connaître les noms des mois de l'année. Il est très peu probable qu'un tel programme ait besoin d'ajouter ou de retirer des mois, donc vous pouvez utiliser un tableau parce que vous savez qu'il contiendra toujours 12 éléments :

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Vous écririez le type d'un tableau en utilisant des crochets, et à l'intérieur des crochets, incluez le type de chaque élément, un point-virgule, puis le nombre d'éléments dans le tableau, comme ceci :

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Ici, `i32` est le type de chaque élément. Après le point-virgule, le nombre `5` indique que le tableau contient cinq éléments.

Écrire le type d'un tableau de cette manière ressemble à une syntaxe alternative pour initialiser un tableau : si vous souhaitez créer un tableau qui contient la même valeur pour chaque élément, vous pouvez spécifier la valeur initiale, suivie d'un point-virgule, puis la longueur du tableau entre crochets, comme indiqué ici :

```rust
let a = [3; 5];
```

Le tableau nommé `a` contiendra `5` éléments qui seront tous initialement définis à la valeur `3`. C'est la même chose que d'écrire `let a = [3, 3, 3, 3, 3];` mais de manière plus concise.

#### Accéder aux éléments du tableau

Un tableau est un bloc mémoire unique alloué sur la pile. Vous pouvez accéder aux éléments d'un tableau en utilisant l'indexation, comme ceci :

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

Dans cet exemple, la variable nommée `first` obtiendra la valeur `1`, car c'est la valeur à l'index `[0]` dans le tableau. La variable nommée `second` obtiendra la valeur `2` de l'index `[1]` dans le tableau.

#### Accès invalide aux éléments du tableau

Que se passe-t-il si vous essayez d'accéder à un élément d'un tableau qui est au-delà de la fin du tableau ? Supposons que vous modifiez l'exemple avec le code suivant, qui se compilera mais produira une erreur à l'exécution :

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Exécuter ce code avec `cargo run` produit le résultat suivant :

```text
   Finished dev [unoptimized + debuginfo] target(s) in 0.05s
   Running `target/debug/Test_Rust_Project`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:5:19
```

La compilation n'a pas produit d'erreurs, mais le programme a entraîné une erreur à _l'exécution_ et n'est pas sorti avec succès. Lorsque vous essayez d'accéder à un élément en utilisant l'indexation, Rust vérifiera que l'index que vous avez spécifié est inférieur à la longueur du tableau. Si l'index est supérieur ou égal à la longueur du tableau, Rust paniquera.

C'est le premier exemple des principes de sécurité de Rust en action. Dans de nombreux langages de bas niveau, ce type de vérification n'est pas effectué, et lorsque vous fournissez un index incorrect, une mémoire invalide peut être accédée. Rust vous protège contre ce type d'erreur en quittant immédiatement plutôt que de permettre l'accès à la mémoire et de continuer. Le chapitre 9 discute plus en détail de la gestion des erreurs en Rust.

_Vous pouvez vous référer au chapitre suivant dans le livre sur le langage de programmation Rust : [Types composés](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#compound-types)_