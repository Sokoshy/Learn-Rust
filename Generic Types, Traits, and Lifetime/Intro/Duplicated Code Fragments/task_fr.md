## Fragments de code dupliqués

Avant de plonger dans la syntaxe des générics, voyons d'abord comment supprimer la duplication qui ne concerne pas les types génériques en extrayant une fonction. Ensuite, nous appliquerons cette technique pour extraire une fonction générique ! De la même manière que vous reconnaissez le code dupliqué à extraire dans une fonction, vous commencerez à reconnaître le code dupliqué qui peut utiliser des générics.

Pour trouver le plus grand nombre dans deux listes différentes de nombres, nous pouvons dupliquer le code ci-dessus et utiliser la même logique à deux endroits différents dans le programme, comme indiqué ci-dessous.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

### Code pour trouver le plus grand nombre dans *deux* listes de nombres

Bien que ce code fonctionne, dupliquer le code est fastidieux et source d'erreurs. Nous devons également mettre à jour le code à plusieurs endroits lorsque nous voulons le modifier.