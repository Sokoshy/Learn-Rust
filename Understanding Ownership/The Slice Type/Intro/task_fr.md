## Le type Slice

Un autre type de données qui n'a pas de possession est le _slice_. Les slices vous permettent de référencer une séquence contiguë d'éléments dans une collection plutôt que la collection entière.

Voici un petit problème de programmation : écrivez une fonction qui prend une chaîne de caractères et renvoie le premier mot qu'elle trouve dans cette chaîne. Si la fonction ne trouve pas d'espace dans la chaîne, la chaîne entière doit être un mot, donc la chaîne entière doit être retournée.

Réfléchissons à la signature de cette fonction :

```rust
    fn first_word(s: &String) -> ?
```

Cette fonction, `first_word`, a un `&String` comme paramètre. Nous ne voulons pas de possession, donc cela convient. Mais que devons-nous retourner ? Nous n'avons pas vraiment de moyen de parler d'une _partie_ d'une chaîne. Cependant, nous pourrions retourner l'indice de fin du mot. Essayons cela, comme le montre l'extrait de code ci-dessous.

```rust
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
```

##### La fonction first_word qui retourne une valeur d'index d'octet dans le paramètre String

Parce que nous devons parcourir l'élément `String` élément par élément et vérifier si une valeur est un espace, nous allons convertir notre `String` en un tableau d'octets en utilisant la méthode `as_bytes` :

```rust
    let bytes = s.as_bytes();
```

Ensuite, nous créons un itérateur sur le tableau d'octets en utilisant la méthode `iter` :

```rust
    for (i, &item) in bytes.iter().enumerate() {
```

Nous discuterons des itérateurs plus en détail au Chapitre 13. Pour l'instant, sachez que `iter` est une méthode qui retourne chaque élément d'une collection et que `enumerate` enveloppe le résultat de `iter` et retourne chaque élément sous forme de tuple. Le premier élément du tuple retourné par `enumerate` est l'indice, et le deuxième élément est une référence à l'élément. C’est un peu plus pratique que de calculer l'indice nous-mêmes.

Comme la méthode `enumerate` retourne un tuple, nous pouvons utiliser des motifs pour déconstruire ce tuple, comme partout ailleurs dans Rust. Ainsi, dans la boucle `for`, nous spécifions un motif qui a `i` pour l'indice dans le tuple et `&item` pour le seul octet dans le tuple. Parce que nous obtenons une référence à l'élément de `.iter().enumerate()`, nous utilisons `&` dans le motif.

À l'intérieur de la boucle `for`, nous recherchons l'octet qui représente l'espace en utilisant la syntaxe littérale d'octet. Si nous trouvons un espace, nous retournons la position. Sinon, nous retournons la longueur de la chaîne en utilisant `s.len()` :

```rust
        if item == b' ' {
            return i;
        }
    }

    s.len()
```

Nous avons maintenant un moyen de trouver l'indice de fin du premier mot dans la chaîne, mais il y a un problème. Nous retournons un `usize` tout seul, mais c’est un nombre significatif uniquement dans le contexte du `&String`. En d'autres termes, parce que c'est une valeur distincte de la `String`, il n'y a aucune garantie qu'elle restera valide à l’avenir. Considérez le programme dans l'extrait de code ci-dessous qui utilise la fonction `first_word` de l'exemple précédent.

```rust
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word obtiendra la valeur 5

        s.clear(); // cela vide la String, la rendant égale à ""

        // word a toujours la valeur 5 ici, mais il n'y a plus de chaîne de
        // laquelle nous pourrions utiliser la valeur 5 de manière significative. word est maintenant complètement invalide !
    }
```

##### Stocker le résultat de l'appel à la fonction first_word puis changer le contenu de la String

Ce programme se compile sans erreurs et le ferait également si nous utilisions `word` après avoir appelé `s.clear()`. Parce que `word` n’est pas lié à l’état de `s`, `word` contient toujours la valeur `5`. Nous pourrions utiliser cette valeur `5` avec la variable `s` pour essayer d'extraire le premier mot, mais ce serait une erreur car le contenu de `s` a changé depuis que nous avons enregistré `5` dans `word`.

Devoir se soucier du fait que l'indice dans `word` peut ne plus être synchronisé avec les données dans `s` est fastidieux et sujet aux erreurs ! Gérer ces indices est encore plus fragile si nous écrivons une fonction `second_word`. Sa signature devrait ressembler à ceci :

```rust
    fn second_word(s: &String) -> (usize, usize) {
```

Nous suivons maintenant un indice de début _et_ un indice de fin, et nous avons encore plus de valeurs qui ont été calculées à partir de données dans un état particulier mais qui ne sont pas liées à cet état. Nous avons maintenant trois variables non reliées les unes aux autres qui doivent être synchronisées.

Heureusement, Rust a une solution à ce problème : les slices de chaînes.