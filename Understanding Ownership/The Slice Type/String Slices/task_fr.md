## Segments de chaîne

Un _segment de chaîne_ est une référence à une partie d'un `String` et cela ressemble à ceci :

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

C'est similaire à prendre une référence à l'ensemble du `String` mais avec la partie supplémentaire `[0..5]`. Plutôt qu'une référence à l'ensemble du `String`, c'est une référence à une portion du `String`.

Nous pouvons créer des segments en utilisant une plage dans des crochets en spécifiant `[indice_de_début..indice_de_fin]`, où `indice_de_début` est la première position dans le segment et `indice_de_fin` est la dernière position dans le segment + 1. En interne, la structure de données du segment stocke la position de départ et la longueur du segment, ce qui correspond à `indice_de_fin` moins `indice_de_début`. Donc, dans le cas de `let world = &s[6..11];`, `world` serait un segment qui contient un pointeur vers le 7ème octet (en comptant à partir de 1) de `s` avec une valeur de longueur de 5.

La figure 6 montre cela dans un diagramme.

<img alt="world contenant un pointeur vers le 6ème octet de la chaîne s et une longueur de 5" src="https://doc.rust-lang.org/stable/book/img/trpl04-06.svg" class="center" style="width: 50%;">

##### Figure 6 : Segment de chaîne se référant à une partie d'une chaîne

Avec la syntaxe de plage `..` de Rust, si vous voulez commencer à l'index zéro, vous pouvez omettre la valeur avant les deux points. Autrement dit, ceux-ci sont équivalents :

```rust
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
```

De la même manière, si votre segment inclut le dernier octet du `String`, vous pouvez omettre le nombre final. Cela signifie que ces lignes sont équivalentes :

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
```

Vous pouvez également omettre les deux valeurs pour prendre un segment de toute la chaîne. Donc ceux-ci sont équivalents :

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

> Remarque : Les indices de plage des segments de chaîne doivent se situer aux frontières valides des caractères UTF-8. Si vous tentez de créer un segment de chaîne au milieu d'un caractère multioctet, votre programme se terminera par une erreur. Aux fins d'introduction aux segments de chaîne, nous supposons uniquement ASCII dans cette section ; une discussion plus approfondie de la gestion UTF-8 se trouve dans la section [« Stocker du texte codé en UTF-8 avec des chaînes »](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) du chapitre "Collections communes".

Avec toutes ces informations à l'esprit, réécrivons `first_word` pour qu'elle retourne un segment. Le type qui signifie "segment de chaîne" s'écrit `&str` :

```rust
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
```

Nous obtenons l'index pour la fin du mot de la même manière que dans le premier code, en cherchant la première occurrence d'un espace. Lorsque nous trouvons un espace, nous renvoyons un segment de chaîne en utilisant le début de la chaîne et l'index de l'espace comme indices de début et de fin.

Maintenant, lorsque nous appelons `first_word`, nous récupérons une seule valeur qui est liée aux données sous-jacentes. La valeur est constituée d'une référence au point de départ du segment et du nombre d'éléments dans le segment.

Retourner un segment fonctionnerait également pour une fonction `second_word` :

```rust
    fn second_word(s: &String) -> &str {
```

Nous avons maintenant une API simple qui est beaucoup plus difficile à rater, car le compilateur s'assurera que les références dans le `String` restent valides. Souvenez-vous du bogue dans le programme de l'exemple "Stocker le résultat de l'appel de la fonction first_word et ensuite changer le contenu de la chaîne", lorsque nous avons obtenu l'index jusqu'à la fin du premier mot mais que nous avons ensuite vidé la chaîne, rendant notre index invalide ? Ce code était logiquement incorrect mais ne montrait aucune erreur immédiate. Les problèmes apparaîtraient plus tard si nous continuions à essayer d'utiliser l'index du premier mot avec une chaîne vidée. Les segments rendent ce bogue impossible et nous informent que nous avons un problème avec notre code beaucoup plus tôt. L'utilisation de la version segment de `first_word` générera une erreur à la compilation :

```rust
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // erreur !

        println!("le premier mot est : {}", word);
    }
```

Voici l'erreur du compilateur :

```text
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/main.rs:18:5
       |
    16 |     let word = first_word(&s);
       |                           -- immutable borrow occurs here
    17 |
    18 |     s.clear(); // erreur !
       |     ^^^^^^^^^ emprunt mutable intervient ici
    19 |
    20 |     println!("le premier mot est : {}", word);
       |                                        ---- emprunt immutable réutilisé ici
```

Rappelez-vous des règles de l'emprunt : si nous avons une référence immuable à quelque chose, nous ne pouvons pas non plus prendre une référence mutable. Parce que `clear` doit tronquer le `String`, il doit obtenir une référence mutable. Rust ne permet pas cela, et la compilation échoue. Non seulement Rust a rendu notre API plus facile à utiliser, mais elle a également éliminé toute une classe d'erreurs à la compilation !