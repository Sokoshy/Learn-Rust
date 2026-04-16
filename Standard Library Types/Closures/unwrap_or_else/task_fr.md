#### Exemple : `unwrap_or_else`

Examinons la définition de la méthode `unwrap_or_else` sur `Option<T>` que nous avons utilisée [précédemment](course://Standard Library Types/Closures/Capturing the Environment with Closures) :

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Rappelez-vous que `T` est le type générique représentant le type de la valeur dans la variante `Some` d’une `Option`. Ce type `T` est également le type de retour de la fonction `unwrap_or_else` : le code qui appelle `unwrap_or_else` sur une `Option<String>`, par exemple, obtiendra un `String`.

Ensuite, notez que la fonction `unwrap_or_else` a un paramètre de type générique supplémentaire, `F`. Le type `F` est le type du paramètre nommé `f`, qui est le closure que nous fournissons lors de l'appel à `unwrap_or_else`.

La contrainte de trait spécifiée sur le type générique `F` est `FnOnce() -> T`, ce qui signifie que `F` doit pouvoir être appelé au moins une fois, ne prendre aucun argument, et retourner un `T`. L’utilisation de `FnOnce` dans la contrainte de trait exprime la restriction qu’`unwrap_or_else` va appeler `f` au maximum une fois. Dans le corps de `unwrap_or_else`, nous pouvons voir que si l’`Option` est `Some`, `f` ne sera pas appelé. Si l’`Option` est `None`, `f` sera appelé une fois. Parce que toutes les closures implémentent `FnOnce`, `unwrap_or_else` accepte le plus grand nombre de types de closures différents et est aussi flexible que possible.

> Remarque : Les fonctions peuvent également implémenter les trois traits `Fn`. Si ce que nous voulons faire ne nécessite pas de capturer une valeur de l'environnement, nous pouvons utiliser le nom d'une fonction plutôt qu'une closure là où nous avons besoin de quelque chose qui implémente un des traits `Fn`. Par exemple, sur une valeur `Option<Vec<T>>`, nous pourrions appeler `unwrap_or_else(Vec::new)` pour obtenir un nouveau vecteur vide si la valeur est `None`.