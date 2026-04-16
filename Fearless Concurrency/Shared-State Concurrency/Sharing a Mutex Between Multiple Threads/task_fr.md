#### Partager un `Mutex<T>` entre plusieurs threads

Maintenant, essayons de partager une valeur entre plusieurs threads en utilisant `Mutex<T>`. Nous allons lancer 10 threads et les faire chacun incrémenter une valeur de compteur de 1, pour que le compteur passe de 0 à 10. Le prochain exemple générera une erreur de compilation, et nous utiliserons cette erreur pour en apprendre davantage sur l'utilisation de `Mutex<T>` et comment Rust nous aide à l'utiliser correctement. Le code suivant est notre exemple de départ :

```rust
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
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

##### Dix threads incrémentent chacun un compteur protégé par un `Mutex<T>`

Nous créons une variable `counter` pour contenir un `i32` à l'intérieur d'un `Mutex<T>`, comme nous l'avons fait dans le premier exemple. Ensuite, nous créons 10 threads en itérant sur une plage de nombres. Nous utilisons `thread::spawn` et donnons à tous les threads la même fermeture, qui déplace le compteur dans le thread, acquiert un verrou sur le `Mutex<T>` en appelant la méthode `lock`, puis ajoute 1 à la valeur dans le mutex. Lorsqu'un thread termine l'exécution de sa fermeture, `num` sortira de la portée et libérera le verrou pour qu'un autre thread puisse l'acquérir.

Dans le thread principal, nous collectons tous les gestionnaires de jointure. Ensuite, nous appelons `join` sur chaque gestionnaire pour nous assurer que tous les threads se terminent. À ce moment-là, le thread principal acquerra le verrou et affichera le résultat de ce programme.

Nous avons suggéré que cet exemple ne compilerait pas. Trouvons maintenant pourquoi !

```text
error[E0382]: use of moved value: `counter`
  --> src/main.rs:9:36
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
10 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure
```

Le message d'erreur indique que la valeur `counter` a été déplacée lors de l'itération précédente de la boucle. Ainsi, Rust nous dit que nous ne pouvons pas déplacer la propriété du verrou `counter` dans plusieurs threads. Corrigeons l'erreur du compilateur avec une méthode de propriété multiple qui est discutée dans [le Chapitre 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) du livre Rust.