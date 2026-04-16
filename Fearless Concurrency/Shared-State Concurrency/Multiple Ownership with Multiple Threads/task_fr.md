### Propriété multiple avec plusieurs threads

Dans le [Chapitre 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) du Livre Rust, les auteurs ont donné à une valeur plusieurs propriétaires en utilisant le pointeur intelligent `Rc<T>` pour créer une valeur avec comptage de références. Faisons de même ici et voyons ce qui se passe. Nous allons envelopper le `Mutex<T>` dans un `Rc<T>` dans l'exemple suivant et cloner le `Rc<T>` avant de transférer la propriété au thread. Maintenant que nous avons vu les erreurs, nous reviendrons également à l'utilisation de la boucle `for`, et nous conserverons le mot-clé `move` avec la fermeture.

```rust
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Rc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Résultat : {}", *counter.lock().unwrap());
    }
```

##### Tenter d'utiliser Rc<T> pour permettre à plusieurs threads de posséder le Mutex<T>

Encore une fois, nous compilons et nous obtenons... des erreurs différentes ! Le compilateur nous apprend beaucoup de choses.

```text
error[E0277]: `Rc<Mutex<i32>>` ne peut pas être envoyé entre les threads en toute sécurité
   --> src/main.rs:11:22
    |
11  |           let handle = thread::spawn(move || {
    |  ______________________^^^^^^^^^^^^^_-
    | |                      |
    | |                      `Rc<Mutex<i32>>` ne peut pas être envoyé entre les threads en toute sécurité
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________- dans ce `[closure@src/main.rs:11:36: 15:10]`
    |
    = help: dans `[closure@src/main.rs:11:36: 15:10]`, le trait `Send` n'est pas implémenté pour `Rc<Mutex<i32>>`
    = note: requis car il apparaît dans le type `[closure@src/main.rs:11:36: 15:10]`
```

Wow, ce message d'erreur est très verbeux ! Voici la partie importante sur laquelle se concentrer : `` `Rc<Mutex<i32>>` ne peut pas être envoyé entre les threads en toute sécurité ``. Le compilateur nous dit également pourquoi : `` le trait `Send` n'est pas implémenté pour `Rc<Mutex<i32>>` ``. Nous parlerons de `Send` dans la section suivante : c'est un des traits qui assure que les types que nous utilisons avec les threads sont destinés à être utilisés dans des situations de concurrence.

Malheureusement, `Rc<T>` n'est pas sûr à partager entre les threads. Lorsqu'un `Rc<T>` gère le comptage de références, il ajoute au compte à chaque appel de `clone` et soustrait du compte lorsque chaque clone est supprimé. Mais il n'utilise pas de primitives de concurrence pour s'assurer que les modifications du compte ne peuvent pas être interrompues par un autre thread. Cela pourrait conduire à des comptes erronés — des bugs subtils qui pourraient à leur tour entraîner des fuites de mémoire ou une valeur étant supprimée avant que nous ayons terminé avec elle. Ce dont nous avons besoin, c'est d'un type exactement comme `Rc<T>`, mais qui modifie le comptage de références de manière sûre pour les threads.