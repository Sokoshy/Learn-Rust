### Le trait Iterator et la méthode next

Tous les itérateurs implémentent un trait nommé `Iterator` qui est défini dans la bibliothèque standard. La définition du trait ressemble à ceci :

```rust
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // méthodes avec implémentations par défaut omises
    }
```

Notez que cette définition utilise une nouvelle syntaxe : `type Item` et `Self::Item`, qui définissent un _type associé_ avec ce trait. Nous parlerons des types associés en détail au Chapitre 19. Pour l'instant, tout ce que vous devez savoir est que ce code indique que l'implémentation du trait `Iterator` requiert également de définir un type `Item`, et ce type `Item` est utilisé dans le type de retour de la méthode `next`. En d'autres termes, le type `Item` sera le type retourné par l'itérateur.

Le trait `Iterator` exige que les implémenteurs définissent une seule méthode : la méthode `next`, qui retourne un élément de l'itérateur à la fois enveloppé dans `Some` et, lorsque l'itération est terminée, retourne `None`.

Nous pouvons appeler directement la méthode `next` sur les itérateurs ; l'exemple suivant démontre les valeurs retournées par les appels répétés à `next` sur l'itérateur créé à partir du vecteur.

```rust
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

##### Appeler la méthode next sur un itérateur

Notez que nous devions rendre `v1_iter` mutable : appeler la méthode `next` sur un itérateur modifie l'état interne que l'itérateur utilise pour suivre où il en est dans la séquence. En d'autres termes, ce code _consomme_, ou utilise, l'itérateur. Chaque appel à `next` utilise un élément de l'itérateur. Nous n'avions pas besoin de rendre `v1_iter` mutable lorsque nous avons utilisé une boucle `for` car la boucle prenait possession de `v1_iter` et le rendait mutable en arrière-plan.

Notez également que les valeurs que nous obtenons des appels à `next` sont des références immuables aux valeurs dans le vecteur. La méthode `iter` produit un itérateur sur des références immuables. Si nous voulons créer un itérateur qui prend possession de `v1` et retourne des valeurs possédées, nous pouvons appeler `into_iter` au lieu de `iter`. De même, si nous voulons itérer sur des références mutables, nous pouvons appeler `iter_mut` au lieu de `iter`.