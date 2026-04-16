### Implémentations par défaut

Parfois, il est utile d'avoir un comportement par défaut pour certaines ou toutes les méthodes d'un trait au lieu d'exiger des implémentations pour toutes les méthodes sur chaque type. Ensuite, lorsque nous implémentons le trait pour un type particulier, nous pouvons conserver ou remplacer le comportement par défaut de chaque méthode.

Le code ci-dessous montre comment spécifier une chaîne de caractères par défaut pour la méthode `summarize` du trait `Summary` au lieu de seulement définir la signature de la méthode, comme nous l'avons fait dans le premier exemple de cette section.

```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Lire plus...)")
    }
}
```

#### Définition d'un trait `Summary` avec une implémentation par défaut de la méthode `summarize`

Pour utiliser une implémentation par défaut pour résumer les instances de `NewsArticle` au lieu de définir une implémentation personnalisée, nous spécifions un bloc `impl` vide avec `impl Summary for NewsArticle {}`.

Même si nous ne définissons plus directement la méthode `summarize` sur `NewsArticle`, nous avons fourni une implémentation par défaut et spécifié que `NewsArticle` implémente le trait `Summary`. En conséquence, nous pouvons toujours appeler la méthode `summarize` sur une instance de `NewsArticle`, comme ceci :

```rust,ignore
let article = NewsArticle {
    headline: String::from("Les manchots remportent le championnat de la Coupe Stanley !"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "Les Penguins de Pittsburgh sont à nouveau la meilleure \
        équipe de hockey de la LNH.",
    ),
};

println!("Nouvel article disponible ! {}", article.summarize());
```

Ce code imprime `Nouvel article disponible ! (Lire plus...)`.

Créer une implémentation par défaut pour `summarize` ne nous oblige pas à modifier quoi que ce soit concernant l'implémentation de `Summary` sur `Tweet` dans le deuxième exemple de cette section. La raison est que la syntaxe pour remplacer une implémentation par défaut est la même que la syntaxe pour implémenter une méthode de trait qui n'a pas d'implémentation par défaut.