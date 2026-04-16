### Créer un nouveau thread avec `spawn`

Pour créer un nouveau thread, nous appelons la fonction `thread::spawn` et lui passons une fermeture (nous avons parlé des fermetures au Chapitre 13) contenant le code que nous voulons exécuter dans le nouveau thread. L'exemple dans l'extrait de code ci-dessous affiche du texte à partir d'un thread principal et un autre texte à partir d'un nouveau thread : 

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("salut numéro {} du thread créé!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("salut numéro {} du thread principal!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

##### Créer un nouveau thread pour imprimer une chose pendant que le thread principal imprime autre chose

Notez qu'avec cette fonction, le nouveau thread sera arrêté lorsque le thread principal se termine, qu'il ait fini ou non son exécution. La sortie de ce programme pourrait être légèrement différente à chaque fois, mais ressemblera à ce qui suit :

```text
    salut numéro 1 du thread principal !
    salut numéro 1 du thread créé !
    salut numéro 2 du thread principal !
    salut numéro 2 du thread créé !
    salut numéro 3 du thread principal !
    salut numéro 3 du thread créé !
    salut numéro 4 du thread principal !
    salut numéro 4 du thread créé !
    salut numéro 5 du thread créé !
```

Les appels à `thread::sleep` forcent un thread à arrêter son exécution pendant une courte durée, permettant à un autre thread de s'exécuter. Les threads prendront probablement des tours, mais cela n'est pas garanti : cela dépend de la manière dont votre système d'exploitation planifie les threads. Dans cette exécution, le thread principal a imprimé en premier, même si l'instruction print du thread créé apparaît en premier dans le code. Et bien que nous ayons demandé au thread créé d'imprimer jusqu'à ce que `i` soit 9, il n'a atteint que 5 avant que le thread principal ne s'arrête.

Si vous exécutez ce code et ne voyez que la sortie du thread principal, ou ne voyez pas de chevauchement, essayez d'augmenter les nombres dans les plages pour créer plus d'opportunités pour le système d'exploitation de basculer entre les threads.