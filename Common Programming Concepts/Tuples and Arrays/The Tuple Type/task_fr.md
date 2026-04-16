## Le type Tuple

Un tuple est une manière générale de regrouper plusieurs valeurs de types divers en un seul type composé. Les tuples ont une longueur fixe : une fois déclarés, ils ne peuvent pas augmenter ou diminuer en taille.

Nous créons un tuple en écrivant une liste de valeurs séparées par des virgules à l'intérieur de parenthèses. Chaque position dans le tuple a un type, et les types des différentes valeurs dans le tuple n'ont pas à être les mêmes. Nous avons ajouté des annotations de type facultatives dans cet exemple :

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

La variable `tup` est liée à l'ensemble du tuple parce qu'un tuple est considéré comme un seul élément composé. Pour obtenir les valeurs individuelles d'un tuple, nous pouvons utiliser le _pattern matching_ pour déstructurer une valeur de tuple, comme ceci :

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("La valeur de y est : {}", y);
}
```

Ce programme crée d'abord un tuple et le lie à la variable `tup`. Il utilise ensuite un modèle avec `let` pour prendre `tup` et le transformer en trois variables distinctes, `x`, `y` et `z`. Cela s'appelle la _déstructuration_, car cela décompose le tuple unique en trois parties. Enfin, le programme affiche la valeur de `y`, qui est `6.4`.

En plus de la déstructuration par le _pattern matching_, nous pouvons accéder directement à un élément d'un tuple en utilisant un point `(.)` suivi de l'indice de la valeur que nous voulons accéder. Par exemple :

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Ce programme crée un tuple, `x`, puis crée de nouvelles variables pour chaque élément en utilisant leurs indices respectifs. Comme dans la plupart des langages de programmation, le premier indice dans un tuple est 0.