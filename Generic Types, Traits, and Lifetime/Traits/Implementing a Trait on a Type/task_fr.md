### Implémentation d'un trait sur un type

Maintenant que nous avons défini le comportement souhaité en utilisant le trait `Summary`, nous pouvons l'implémenter sur les types de notre agrégateur de médias. L'exemple ci-dessous montre une implémentation du trait `Summary` sur la structure `NewsArticle` qui utilise le titre, l'auteur et le lieu pour créer la valeur de retour de `summarize`. Pour la structure `Tweet`, nous définissons `summarize` comme le nom d'utilisateur suivi du texte intégral du tweet, en supposant que le contenu du tweet est déjà limité à 280 caractères.

```rust,noplayground
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, par {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Implémentation du trait `Summary` sur les types `NewsArticle` et `Tweet`.

Implémenter un trait sur un type est similaire à l'implémentation de méthodes régulières. La différence est qu'après `impl`, nous mettons le nom du trait que nous voulons implémenter, puis utilisons le mot-clé `for`, et ensuite spécifions le nom du type pour lequel nous voulons implémenter le trait. Dans le bloc `impl`, nous mettons les signatures de méthode que la définition du trait a définies. Au lieu d'ajouter un point-virgule après chaque signature, nous utilisons des accolades et remplissons le corps de la méthode avec le comportement spécifique que nous voulons que les méthodes du trait aient pour le type particulier.

Après avoir implémenté le trait, nous pouvons appeler les méthodes sur les instances de `NewsArticle` et `Tweet` de la même manière que nous appelons des méthodes régulières, comme ceci :

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "bien sûr, comme vous le savez probablement déjà, les gens",
    ),
    reply: false,
    retweet: false,
};

println!("1 nouveau tweet: {}", tweet.summarize());
```

Ce code affiche `1 nouveau tweet: horse_ebooks: bien sûr, comme vous le savez probablement déjà, les gens`.

Notez que comme nous avons défini le trait `Summary` et les types `NewsArticle` et `Tweet` dans le même *lib.rs* dans l'exemple "Implémentation du trait `Summary` sur les types `NewsArticle` et `Tweet`", ils sont tous dans le même scope. Disons que ce *lib.rs* est pour une crate que nous avons appelée `aggregator` et que quelqu'un d'autre veut utiliser la fonctionnalité de notre crate pour implémenter le trait `Summary` sur une struct définie dans le scope de leur bibliothèque. Ils devraient d'abord introduire le trait dans leur scope. Ils le feraient en spécifiant `use aggregator::Summary;`, ce qui leur permettrait ensuite d'implémenter `Summary` pour leur type. Le trait `Summary` devrait également être un trait public pour qu'une autre crate puisse l'implémenter, ce qu'il est puisque nous avons placé le mot-clé `pub` avant `trait` dans l'exemple 10-12.

Une restriction à noter avec les implémentations de traits est que nous pouvons implémenter un trait sur un type uniquement si soit le trait soit le type est local à notre crate. Par exemple, nous pouvons implémenter les traits de la bibliothèque standard comme `Display` sur un type personnalisé comme `Tweet` dans le cadre de la fonctionnalité de notre crate `aggregator`, parce que le type `Tweet` est local à notre crate `aggregator`. Nous pouvons également implémenter `Summary` sur `Vec<T>` dans notre crate `aggregator`, parce que le trait `Summary` est local à notre crate `aggregator`.

Mais nous ne pouvons pas implémenter des traits externes sur des types externes. Par exemple, nous ne pouvons pas implémenter le trait `Display` sur `Vec<T>` dans notre crate `aggregator`, parce que `Display` et `Vec<T>` sont définis dans la bibliothèque standard et ne sont pas locaux à notre crate `aggregator`. Cette restriction fait partie d'une propriété des programmes appelée *cohérence*, et plus spécifiquement la *règle de l'orphelin*, ainsi appelée parce que le type parent n'est pas présent. Cette règle garantit que le code des autres ne peut pas casser votre code et vice versa. Sans la règle, deux crates pourraient implémenter le même trait pour le même type, et Rust ne saurait pas quelle implémentation utiliser.