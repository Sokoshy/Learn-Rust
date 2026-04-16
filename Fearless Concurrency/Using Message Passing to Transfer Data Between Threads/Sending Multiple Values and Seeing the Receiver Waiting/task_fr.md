### Envoyer plusieurs valeurs et voir le récepteur attendre

Le code dans l'extrait concernant la réception de "hi" dans le thread principal a été compilé et exécuté, mais il ne montrait pas clairement que deux threads distincts communiquaient entre eux via le canal. Dans l'extrait suivant, nous avons apporté quelques modifications qui prouveront que le code s'exécute de manière concurrente : le thread créé enverra désormais plusieurs messages et fera une pause d'une seconde entre chaque message.

```rust
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
```

##### Envoyer plusieurs messages et faire une pause entre chacun

Cette fois-ci, le thread créé a un vecteur de chaînes de caractères que nous voulons envoyer au thread principal. Nous itérons sur elles, les envoyant individuellement, et faisons une pause entre chacune en appelant la fonction `thread::sleep` avec une valeur `Duration` de 1 seconde.

Dans le thread principal, nous n'appelons plus explicitement la fonction `recv` : à la place, nous traitons `rx` comme un itérateur. Pour chaque valeur reçue, nous l'imprimons. Lorsque le canal est fermé, l'itération se termine.

Lors de l'exécution du code dans le dernier exemple, vous devriez voir la sortie suivante avec une pause d'une seconde entre chaque ligne :

```text
    Got: hi
    Got: from
    Got: the
    Got: thread
```

Comme nous n'avons pas de code qui fait une pause ou un délai dans la boucle `for` du thread principal, nous pouvons déduire que le thread principal attend de recevoir des valeurs du thread créé.