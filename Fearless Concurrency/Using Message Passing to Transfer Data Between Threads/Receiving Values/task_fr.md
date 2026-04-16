## Recevoir des valeurs

Dans le prochain exemple, nous allons obtenir la valeur à l'extrémité réceptrice du canal dans le fil d'exécution principal. C'est comme récupérer le canard en plastique de l'eau à la fin de la rivière ou comme recevoir un message de chat.

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

##### Recevoir la valeur "hi" dans le fil principal et l'imprimer

L'extrémité réceptrice d'un canal dispose de deux méthodes utiles : `recv` et `try_recv`. Nous utilisons `recv`, qui signifie _receive_ (recevoir), ce qui bloquera l'exécution du fil principal et attendra qu'une valeur soit envoyée par le canal. Une fois qu'une valeur est envoyée, `recv` la renverra sous la forme d'un `Result<T, E>`. Lorsque l'extrémité émettrice du canal est fermée, `recv` renverra une erreur pour indiquer qu'il n'y aura plus de valeurs à venir.

La méthode `try_recv` ne bloque pas, mais renverra immédiatement un `Result<T, E>` : une valeur `Ok` contenant un message si un est disponible et une valeur `Err` s'il n'y a pas de message cette fois-ci. Utiliser `try_recv` est utile si ce fil a d'autres tâches à accomplir en attendant les messages : nous pourrions écrire une boucle qui appelle `try_recv` de temps en temps, gère un message si un est disponible, et sinon fait d'autres travaux pendant un certain temps avant de vérifier à nouveau.

Nous avons utilisé `recv` dans cet exemple pour plus de simplicité ; nous n'avons pas d'autre tâche pour le fil principal à part attendre les messages, donc bloquer le fil principal est approprié.

Lorsque nous exécutons le code dans l'extrait ci-dessous, nous verrons la valeur imprimée depuis le fil principal :

```text
    Got: hi
```

Parfait !