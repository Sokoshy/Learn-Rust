### Méthodes qui Consomment l'Iterateur

Le trait `Iterator` possède un certain nombre de méthodes différentes avec des implémentations par défaut fournies par la bibliothèque standard ; vous pouvez découvrir ces méthodes en consultant la documentation API de la bibliothèque standard pour le trait `Iterator`. Certaines de ces méthodes appellent la méthode `next` dans leur définition, c'est pourquoi il est nécessaire d'implémenter la méthode `next` lors de l'implémentation du trait `Iterator`.

Les méthodes qui appellent `next` sont appelées _adaptateurs consommateurs_ car les appeler utilise l'itérateur. Un exemple est la méthode `sum`, qui prend possession de l'itérateur et parcourt les éléments en appelant répétitivement `next`, consommant ainsi l'itérateur. Au fur et à mesure qu'il parcourt les éléments, il les ajoute à un total courant et renvoie le total lorsque l'itération est terminée. Le code ci-dessous montre un test illustrant l'utilisation de la méthode `sum` :

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

##### Appeler la méthode sum pour obtenir le total de tous les éléments de l'itérateur

Nous ne sommes pas autorisés à utiliser `v1_iter` après l'appel à `sum` parce que `sum` prend possession de l'itérateur sur lequel nous l'appelons.