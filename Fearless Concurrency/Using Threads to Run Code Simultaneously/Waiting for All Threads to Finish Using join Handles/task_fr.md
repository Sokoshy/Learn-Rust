### Attendre que tous les threads se terminent en utilisant les handle de jonction

Le code ci-dessus non seulement arrête prématurément le thread lancé la plupart du temps en raison de la fin du thread principal, mais ne peut également garantir que le thread lancé s'exécutera du tout. La raison est qu'il n'y a aucune garantie sur l'ordre dans lequel les threads s'exécutent !

Nous pouvons résoudre le problème du thread lancé qui ne s'exécute pas, ou ne s'exécute pas complètement, en sauvegardant la valeur de retour de `thread::spawn` dans une variable. Le type de retour de `thread::spawn` est `JoinHandle`. Un `JoinHandle` est une valeur possédée qui, lorsque nous appelons la méthode `join` dessus, attendra que son thread se termine. L'exemple ci-dessous montre comment utiliser le `JoinHandle` du thread que nous avons créé auparavant et appeler `join` pour s'assurer que le thread lancé se termine avant que `main` ne se termine :

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("salut numéro {} depuis le thread lancé!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("salut numéro {} depuis le thread principal!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
```

##### Sauvegarder un JoinHandle à partir de thread::spawn pour garantir que le thread s'exécute jusqu'à la fin

Appeler `join` sur le handle bloque le thread actuellement en cours d'exécution jusqu'à ce que le thread représenté par le handle se termine. Un thread _bloqué_ signifie que ce thread est empêché de faire du travail ou de se terminer. Parce que nous avons placé l'appel à `join` après la boucle `for` du thread principal, l'exécution de ce code devrait produire une sortie semblable à celle-ci :

```text
    salut numéro 1 depuis le thread principal !
    salut numéro 2 depuis le thread principal !
    salut numéro 1 depuis le thread lancé !
    salut numéro 3 depuis le thread principal !
    salut numéro 2 depuis le thread lancé !
    salut numéro 4 depuis le thread principal !
    salut numéro 3 depuis le thread lancé !
    salut numéro 4 depuis le thread lancé !
    salut numéro 5 depuis le thread lancé !
    salut numéro 6 depuis le thread lancé !
    salut numéro 7 depuis le thread lancé !
    salut numéro 8 depuis le thread lancé !
    salut numéro 9 depuis le thread lancé !
```

Les deux threads continuent d'alterner, mais le thread principal attend à cause de l'appel à `handle.join()` et ne se termine pas tant que le thread lancé n'est pas fini.

Mais voyons ce qui se passe lorsque nous déplaçons `handle.join()` avant la boucle `for` dans `main`, comme ceci :

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("salut numéro {} depuis le thread lancé!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("salut numéro {} depuis le thread principal !", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

Le thread principal attendra que le thread lancé se termine puis exécutera sa boucle `for`, donc la sortie ne sera plus entremêlée, comme montré ici :

```text
    salut numéro 1 depuis le thread lancé !
    salut numéro 2 depuis le thread lancé !
    salut numéro 3 depuis le thread lancé !
    salut numéro 4 depuis le thread lancé !
    salut numéro 5 depuis le thread lancé !
    salut numéro 6 depuis le thread lancé !
    salut numéro 7 depuis le thread lancé !
    salut numéro 8 depuis le thread lancé !
    salut numéro 9 depuis le thread lancé !
    salut numéro 1 depuis le thread principal !
    salut numéro 2 depuis le thread principal !
    salut numéro 3 depuis le thread principal !
    salut numéro 4 depuis le thread principal !
```

Les petits détails, comme l'endroit où `join` est appelé, peuvent influencer si vos threads s'exécutent en même temps ou non.