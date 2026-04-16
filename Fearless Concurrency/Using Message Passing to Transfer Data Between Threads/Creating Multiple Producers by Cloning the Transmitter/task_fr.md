### Création de plusieurs producteurs en clonant le transmetteur

Plus tôt, nous avons mentionné que `mpsc` était un acronyme pour _multiple producer, single consumer_ (plusieurs producteurs, un seul consommateur). Mettons `mpsc` en pratique et développons le code de l'extrait précédent pour créer plusieurs threads qui envoient tous des valeurs au même récepteur. Nous pouvons le faire en clonant la moitié émettrice du canal, comme indiqué ci-dessous :

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("salut"),
            String::from("de"),
            String::from("la"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("plus"),
            String::from("de messages"),
            String::from("pour"),
            String::from("toi"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Reçu: {}", received);
    }

    // --snip--
```

##### Envoyer plusieurs messages depuis plusieurs producteurs

Cette fois-ci, avant de créer le premier thread engendré, nous appelons `clone` sur l'extrémité émettrice du canal. Cela nous donnera un nouvel identifiant d'envoi que nous pouvons passer au premier thread engendré. Nous passons l'extrémité d'envoi originale du canal à un second thread engendré. Cela nous donne deux threads, chacun envoyant des messages différents à l'extrémité réceptrice du canal.

Lorsque vous exécutez le code, votre sortie devrait ressembler à ceci :

```text
    Reçu: salut
    Reçu: plus
    Reçu: de
    Reçu: messages
    Reçu: pour
    Reçu: la
    Reçu: thread
    Reçu: toi
```

Vous pourriez voir les valeurs dans un ordre différent ; cela dépend de votre système. C'est ce qui rend la concurrence à la fois intéressante et difficile. Si vous expérimentez avec `thread::sleep`, en lui donnant diverses valeurs dans les différents threads, chaque exécution sera plus non déterministe et produira un résultat différent à chaque fois.

Maintenant que nous avons vu comment fonctionnent les canaux, regardons une autre méthode de concurrence.

Vous pouvez vous référer au chapitre suivant dans le livre _The Rust Programming Language_ : _[Utilisation du passage de messages pour transférer des données entre les threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)_