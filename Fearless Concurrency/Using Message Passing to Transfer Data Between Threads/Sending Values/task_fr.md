### Déplacement d'une extrémité émettrice

Déplaçons l'extrémité émettrice dans un thread engendré et faisons en sorte qu'elle envoie une chaîne de caractères afin que le thread engendré communique avec le thread principal, comme illustré dans l'extrait de code ci-dessous. C'est comme mettre un canard en plastique dans la rivière en amont ou envoyer un message de chat d'un thread à un autre.

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    }
```

##### Déplacement de tx vers un thread engendré et envoi de “hi”

Encore une fois, nous utilisons `thread::spawn` pour créer un nouveau thread et utilisons ensuite `move` pour déplacer `tx` dans la fermeture afin que le thread engendré possède `tx`. Le thread engendré doit posséder l'extrémité émettrice du canal pour pouvoir envoyer des messages à travers le canal.

L'extrémité émettrice dispose d'une méthode `send` qui prend la valeur que nous voulons envoyer. La méthode `send` renvoie un type `Result<T, E>`, donc si l'extrémité réceptrice a déjà été supprimée et qu'il n'y a nulle part où envoyer une valeur, l'opération d'envoi renverra une erreur. Dans cet exemple, nous appelons `unwrap` pour engendrer une panique en cas d'erreur. Mais dans une application réelle, nous la gérerions correctement : retournez au Chapitre 9 pour revoir les stratégies de gestion appropriée des erreurs.