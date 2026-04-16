### Définir un trait

Le comportement d'un type consiste en les méthodes que nous pouvons appeler sur ce type. Différents types partagent le même comportement si nous pouvons appeler les mêmes méthodes sur tous ces types. Les définitions de traits sont un moyen de regrouper des signatures de méthodes ensemble pour définir un ensemble de comportements nécessaires pour accomplir un certain but.

Par exemple, disons que nous avons plusieurs structures qui contiennent différents types et quantités de texte : une structure `NewsArticle` qui contient une histoire d'actualité classée dans un endroit particulier et un `Tweet` qui peut avoir au maximum 280 caractères avec des métadonnées indiquant s'il s'agissait d'un nouveau tweet, d'un retweet ou d'une réponse à un autre tweet.

Nous voulons créer une bibliothèque d'agrégation de médias qui peut afficher des résumés de données susceptibles d'être stockées dans une instance `NewsArticle` ou `Tweet`. Pour ce faire, nous avons besoin d'un résumé de chaque type, et nous devons demander ce résumé en appelant une méthode `summarize` sur une instance. Le fragment de code ci-dessous montre la définition d'un trait `Summary` qui exprime ce comportement.

```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### Un trait `Summary` qui consiste en le comportement fourni par une méthode `summarize`

Ici, nous déclarons un trait en utilisant le mot-clé `trait` puis le nom du trait, qui est `Summary` dans ce cas. À l'intérieur des accolades, nous déclarons les signatures des méthodes qui décrivent les comportements des types qui implémentent ce trait, qui dans ce cas est `fn summarize(&self) -> String`.

Après la signature de la méthode, au lieu de fournir une implémentation à l'intérieur des accolades, nous utilisons un point-virgule. Chaque type implémentant ce trait doit fournir son propre comportement personnalisé pour le corps de la méthode. Le compilateur imposera que tout type ayant le trait `Summary` aura la méthode `summarize` définie avec cette signature exactement.

Un trait peut avoir plusieurs méthodes dans son corps : les signatures de méthode sont listées une par ligne et chaque ligne se termine par un point-virgule.