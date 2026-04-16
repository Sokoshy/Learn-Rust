### Appeler des méthodes sans implémentations par défaut

Les implémentations par défaut peuvent appeler d'autres méthodes dans le même trait, même si ces autres méthodes n'ont pas d'implémentation par défaut. De cette façon, un trait peut fournir beaucoup de fonctionnalités utiles et ne nécessite des implémenteurs que de spécifier une petite partie de celle-ci. Par exemple, nous pourrions définir le trait `Summary` pour avoir une méthode `summarize_author` dont l'implémentation est requise, puis définir une méthode `summarize` qui a une implémentation par défaut qui appelle la méthode `summarize_author` :

```rust,noplayground
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Lire plus de {}...)", self.summarize_author())
    }
}
```

Pour utiliser cette version de `Summary`, nous avons seulement besoin de définir `summarize_author` lorsque nous implémentons le trait sur un type :

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Après avoir défini `summarize_author`, nous pouvons appeler `summarize` sur des instances de la structure `Tweet`, et l'implémentation par défaut de `summarize` appellera la définition de `summarize_author` que nous avons fournie. Parce que nous avons implémenté `summarize_author`, le trait `Summary` nous a donné le comportement de la méthode `summarize` sans nous demander d'écrire plus de code.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "bien sûr, comme vous le savez probablement déjà, les gens",
    ),
    reply: false,
    retweet: false,
};

println!("1 nouveau tweet : {}", tweet.summarize());
```

Ce code affiche `1 nouveau tweet : (Lire plus de @horse_ebooks...)`.

Notez qu'il n'est pas possible d'appeler l'implémentation par défaut à partir d'une implémentation qui surcharge cette même méthode.