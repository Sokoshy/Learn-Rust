#### L'API de Mutex<T>

À titre d'exemple sur l'utilisation d'un mutex, commençons par utiliser un mutex dans un contexte mono-threadé, comme illustré ci-dessous :

```rust
    use std::sync::Mutex;

    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
```

##### Explorer l'API de Mutex<T> dans un contexte mono-threadé pour la simplicité

Comme pour de nombreux types, nous créons un `Mutex<T>` en utilisant la fonction associée `new`. Pour accéder aux données à l'intérieur du mutex, nous utilisons la méthode `lock` pour acquérir le verrou. Cet appel bloquera le thread en cours afin qu'il ne puisse rien faire tant que ce n'est pas notre tour d'avoir le verrou.

L'appel à `lock` échouerait si un autre thread détenant le verrou a paniqué. Dans ce cas, personne ne pourrait jamais obtenir le verrou, donc nous avons choisi d'utiliser `unwrap` et de faire paniquer ce thread si nous sommes dans cette situation.

Après avoir acquis le verrou, nous pouvons traiter la valeur de retour, nommée `num` dans ce cas, comme une référence mutable aux données internes. Le système de types garantit que nous acquérons un verrou avant d'utiliser la valeur dans `m`: `Mutex<i32>` n'est pas un `i32`, donc nous _devons_ acquérir le verrou pour pouvoir utiliser la valeur `i32`. Nous ne pouvons pas oublier; le système de types ne nous permettre pas d'accéder au `i32` interne sinon.

Comme vous pouvez le suspecter, `Mutex<T>` est un pointeur intelligent. Plus précisément, l'appel à `lock` _retourne_ un pointeur intelligent appelé `MutexGuard`, enveloppé dans un `LockResult` que nous avons traité avec l'appel à `unwrap`. Le pointeur intelligent `MutexGuard` implémente `Deref` pour pointer vers nos données internes; le pointeur intelligent a également une implémentation `Drop` qui libère automatiquement le verrou lorsqu'un `MutexGuard` sort de la portée, ce qui se produit à la fin de la portée interne dans l'extrait de code précédent. En conséquence, nous ne risquons pas d'oublier de libérer le verrou et de bloquer l'utilisation du mutex par d'autres threads car la libération du verrou se fait automatiquement.

Après avoir libéré le verrou, nous pouvons imprimer la valeur du mutex et voir que nous avons pu changer le `i32` interne à 6.