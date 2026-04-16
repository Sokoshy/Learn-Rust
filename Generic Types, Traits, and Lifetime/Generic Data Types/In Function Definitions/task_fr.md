### Dans les définitions de fonction

Lors de la définition d'une fonction qui utilise des génériques, nous plaçons les génériques dans la signature de la fonction où nous indiquerions normalement les types de données des paramètres et la valeur de retour. Cela rend notre code plus flexible et offre plus de fonctionnalités aux appelants de notre fonction tout en évitant la duplication du code.

En continuant avec notre fonction `largest`, l'extrait ci-dessous montre deux fonctions qui trouvent toutes deux la plus grande valeur dans une tranche.

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
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

    let result = largest_i32(&number_list);
    println!("Le nombre le plus grand est {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("Le caractère le plus grand est {}", result);
}
```

#### Deux fonctions qui diffèrent uniquement par leurs noms et les types dans leurs signatures

La fonction `largest_i32` est celle que nous avons extraite dans le dernier extrait de code de la section précédente, qui trouve le plus grand `i32` dans une tranche. La fonction `largest_char` trouve le plus grand `char` dans une tranche. Les corps des fonctions contiennent le même code, nous allons donc éliminer la duplication en introduisant un paramètre de type générique dans une seule fonction.

Pour paramétrer les types dans la nouvelle fonction que nous définirons, nous devons nommer le paramètre de type, tout comme nous le faisons pour les paramètres de valeur d'une fonction. Vous pouvez utiliser n'importe quel identifiant comme nom de paramètre de type. Mais nous utiliserons `T` car, par convention, les noms de paramètres en Rust sont courts, souvent juste une lettre, et la convention de nommage de types en Rust est CamelCase. Court pour "type", `T` est le choix par défaut de la plupart des programmeurs Rust.

Lorsque nous utilisons un paramètre dans le corps de la fonction, nous devons déclarer le nom du paramètre dans la signature, afin que le compilateur sache ce que signifie ce nom. De même, lorsque nous utilisons un nom de paramètre de type dans une signature de fonction, nous devons déclarer le nom du paramètre de type avant de l'utiliser. Pour définir la fonction générique `largest`, placez les déclarations de nom de type dans des crochets angulaires, `<>`, entre le nom de la fonction et la liste des paramètres, comme ceci :

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

Nous lisons cette définition ainsi : la fonction `largest` est générique par rapport à un type `T`. Cette fonction a un paramètre nommé `list`, qui est une tranche de valeurs de type `T`. La fonction `largest` renverra une référence à une valeur du même type `T`.

L'extrait de code ci-dessous montre la définition combinée de la fonction `largest` en utilisant le type de données générique dans sa signature. La liste montre également comment nous pouvons appeler la fonction avec soit une tranche de valeurs `i32` soit de valeurs `char`. Notez que ce code ne se compilera pas encore, mais nous le corrigerons plus tard dans ce chapitre.

```rust,ignore,does_not_compile
fn largest<T>(list: &[T]) -> &T {
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
    println!("Le nombre le plus grand est {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("Le caractère le plus grand est {}", result);
}
```

#### Une définition de la fonction `largest` qui utilise des paramètres de type générique mais ne compile pas encore

Si nous compilons ce code maintenant, nous obtiendrons cette erreur :

```console
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

La note mentionne `std::cmp::PartialOrd`, qui est un *trait*. Nous parlerons des traits dans la section suivante. Pour le moment, cette erreur indique que le corps de `largest` ne fonctionnera pas pour tous les types possibles que `T` pourrait être. Parce que nous voulons comparer des valeurs de type `T` dans le corps, nous ne pouvons utiliser que des types dont les valeurs peuvent être ordonnées. Pour permettre les comparaisons, la bibliothèque standard a le trait `std::cmp::PartialOrd` que vous pouvez implémenter sur les types (voir l'Annexe C pour plus d'informations sur ce trait). Vous apprendrez comment spécifier qu'un type générique a un trait particulier dans la partie "Traits comme paramètres" de la leçon suivante, mais explorons d'abord les autres façons d'utiliser les paramètres de type générique.