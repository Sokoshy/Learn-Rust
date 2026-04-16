### Retourner des types qui implémentent des traits

Nous pouvons également utiliser la syntaxe `impl Trait` dans la position de retour pour renvoyer une valeur d'un certain type qui implémente un trait, comme illustré ici :

```rust,ignore
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

En utilisant `impl Summary` pour le type de retour, nous spécifions que la fonction `returns_summarizable` renvoie un certain type qui implémente le trait `Summary` sans nommer le type concret. Dans ce cas, `returns_summarizable` renvoie un `Tweet`, mais le code appelant cette fonction ne le sait pas.

La possibilité de renvoyer un type spécifié uniquement par le trait qu'il implémente est particulièrement utile dans le contexte des fermetures et des itérateurs, que nous couvrons dans le Chapitre 13. Les fermetures et les itérateurs créent des types que seul le compilateur connaît ou des types très longs à spécifier. La syntaxe `impl Trait` vous permet de spécifier de manière concise qu'une fonction renvoie un certain type qui implémente le trait `Iterator` sans avoir besoin d'écrire un type extrêmement long.

Cependant, vous ne pouvez utiliser `impl Trait` que si vous renvoyez un seul type. Par exemple, ce code qui renvoie soit un `NewsArticle` soit un `Tweet` avec le type de retour spécifié comme `impl Summary` ne fonctionnerait pas :

```rust,ignore,does_not_compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

Renvoyer soit un `NewsArticle` soit un `Tweet` n'est pas autorisé en raison des restrictions sur la façon dont la syntaxe `impl Trait` est implémentée dans le compilateur. Nous expliquerons comment écrire une fonction avec ce comportement dans la section [« Utilisation des objets de trait qui permettent des valeurs de différents types »][using-trait-objects-that-allow-for-values-of-different-types] du Chapitre 17 du Rust Book.

[using-trait-objects-that-allow-for-values-of-different-types]:
https://doc.rust-lang.org/book/ch17-02-trait-objects.html