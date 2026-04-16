### Canaux et transfert de propriété

Les règles de propriété jouent un rôle essentiel dans l'envoi de messages car elles vous aident à écrire du code sûr et concurrent. Empêcher les erreurs dans la programmation concurrente est l'avantage de réfléchir à la propriété tout au long de vos programmes Rust. Faisons une expérience pour montrer comment les canaux et la propriété travaillent ensemble pour éviter les problèmes : nous allons essayer d'utiliser une valeur `val` dans le fil d'exécution créé _après_ l'avoir envoyée via le canal. Essayez de compiler le code dans l'exemple suivant pour voir pourquoi ce code n'est pas autorisé :

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

##### Tentative d'utilisation de val après l'avoir envoyée via le canal

Ici, nous essayons d'imprimer `val` après l'avoir envoyée via le canal avec `tx.send`. Permettre cela serait une mauvaise idée : une fois que la valeur a été envoyée à un autre fil d'exécution, celui-ci pourrait la modifier ou la supprimer avant que nous essayions de l'utiliser à nouveau. Il est possible que les modifications de l'autre fil d'exécution causent des erreurs ou des résultats inattendus en raison de données incohérentes ou inexistantes. Cependant, Rust nous donne une erreur si nous essayons de compiler le code dans l'exemple ci-dessous :

```text
    error[E0382]: borrow of moved value: `val`
      --> src/main.rs:10:31
       |
    8  |         let val = String::from("hi");
       |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    9  |         tx.send(val).unwrap();
       |                 --- value moved here
    10 |         println!("val is {}", val);
       |                               ^^^ value borrowed here after move

```

Notre erreur de concurrence a causé une erreur de compilation. La fonction `send` prend la propriété de son paramètre, et lorsque la valeur est déplacée, le récepteur en prend la propriété. Cela nous empêche d'utiliser la valeur à nouveau par accident après l'envoi ; le système de propriété vérifie que tout est en ordre.