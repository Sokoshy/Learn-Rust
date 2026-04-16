## Calcul du plus grand nombre

Considérons un programme court qui trouve le plus grand nombre dans une liste, comme illustré dans l'extrait de code ci-dessous.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("Le plus grand nombre est {}", largest);
}
```

#### Code pour trouver le plus grand nombre dans une liste de nombres.

Ce code stocke une liste d'entiers dans la variable `number_list` et place le premier nombre de la liste dans une variable nommée `largest`. Ensuite, il parcourt tous les nombres de la liste, et si le nombre courant est supérieur au nombre stocké dans `largest`, il remplace le nombre dans cette variable. Cependant, si le nombre actuel est inférieur ou égal au plus grand nombre rencontré jusqu'à présent, la variable ne change pas, et le code passe au nombre suivant de la liste. Après avoir considéré tous les nombres de la liste, `largest` devrait contenir le plus grand nombre, qui dans ce cas est 100.