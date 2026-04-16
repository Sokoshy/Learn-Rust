### Utilisation des fermetures move avec des threads

La fermeture `move` est souvent utilisée avec `thread::spawn` car elle permet d'utiliser des données d'un thread dans un autre thread.

Dans le chapitre 13, nous avons mentionné que nous pouvons utiliser le mot-clé `move` avant la liste des paramètres d'une fermeture pour forcer la fermeture à prendre possession des valeurs qu'elle utilise dans son environnement. Cette technique est particulièrement utile lors de la création de nouveaux threads afin de transférer la propriété des valeurs d'un thread à un autre.

Notez dans l'extrait concernant la création d'un thread que la fermeture que nous passons à `thread::spawn` ne prend aucun argument : nous n'utilisons aucune donnée du thread principal dans le code du thread lancé. Pour utiliser des données du thread principal dans le thread lancé, la fermeture du thread lancé doit capturer les valeurs dont elle a besoin. L'extrait de code ci-dessous montre une tentative de création d'un vecteur dans le thread principal et de son utilisation dans le thread lancé. Cependant, cela ne fonctionnera pas encore, comme vous le verrez dans un instant.

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Voici un vecteur : {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### Tentative d'utilisation d'un vecteur créé par le thread principal dans un autre thread

La fermeture utilise `v`, elle va donc capturer `v` et en faire partie de l'environnement de la fermeture. Étant donné que `thread::spawn` exécute cette fermeture dans un nouveau thread, nous devrions être capables d'accéder à `v` dans ce nouveau thread. Mais lorsque nous compilons cet exemple, nous obtenons l'erreur suivante :

```text
    error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
     --> src/main.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    note: function requires argument type to outlive `'static`
     --> src/main.rs:6:18
      |
    6 |       let handle = thread::spawn(|| {
      |  __________________^
    7 | |         println!("Here's a vector: {:?}", v);
    8 | |     });
      | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

Rust _infère_ comment capturer `v`, et parce que `println!` ne nécessite qu'une référence à `v`, la fermeture essaie d'emprunter `v`. Cependant, il y a un problème : Rust ne peut pas déterminer combien de temps le thread lancé s'exécutera, il ne sait donc pas si la référence à `v` sera toujours valide.

L'exemple suivant fournit un scénario où il est probable que la référence à `v` ne sera pas valide :

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Voici un vecteur : {:?}", v);
        });

        drop(v); // oh non !

        handle.join().unwrap();
    }
```

##### Un thread avec une fermeture qui tente de capturer une référence à `v` depuis un thread principal qui supprime v

Si nous étions autorisés à exécuter ce code, il y a une possibilité que le thread lancé soit immédiatement mis en arrière-plan sans s'exécuter du tout. Le thread lancé a une référence à `v` en interne, mais le thread principal supprime immédiatement `v`, en utilisant la fonction `drop` dont nous avons parlé au chapitre 15. Ensuite, lorsque le thread lancé commence à s'exécuter, `v` n'est plus valide, donc une référence à celui-ci est également invalide. Oh non !

Pour corriger l'erreur du compilateur dans l'exemple du vecteur passé entre les threads, nous pouvons suivre le conseil du message d'erreur :

```text
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

En ajoutant le mot-clé `move` avant la fermeture, nous forçons la fermeture à prendre possession des valeurs qu'elle utilise plutôt que de laisser Rust inférer qu'elle devrait emprunter les valeurs. La modification de ce code montrée dans l'extrait de code ci-dessous se compile et s'exécute comme nous le souhaitons :

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Voici un vecteur : {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### Utilisation du mot-clé move pour forcer une fermeture à prendre possession des valeurs qu'elle utilise

Que se passerait-il dans le code de l'extrait concernant le thread avec une fermeture où le thread principal appelle `drop` si nous utilisons une fermeture `move` ? Est-ce que `move` réglerait ce cas ? Malheureusement, non ; nous obtiendrions une erreur différente car ce que ce code essaie de faire n'est pas autorisé pour une autre raison. Si nous ajoutons `move` à la fermeture, nous déplacerions `v` dans l'environnement de la fermeture, et nous ne pourrions plus appeler `drop` sur celui-ci dans le thread principal. Nous obtiendrions alors l'erreur du compilateur suivante :

```text
    error[E0382]: use of moved value: `v`
      --> src/main.rs:10:10
       |
    4  |     let v = vec![1, 2, 3];
       |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    5  | 
    6  |     let handle = thread::spawn(move || {
       |                                ------- value moved into closure here
    7  |         println!("Here's a vector: {:?}", v);
       |                                           - variable moved due to use in closure
    ...
    10 |     drop(v); // oh non !
       |          ^ value used here after move
```

Les règles de possession de Rust nous ont encore une fois sauvés ! Nous avons eu une erreur du code concernant le passage d'un vecteur entre les threads parce que Rust était conservateur et ne faisaient qu'emprunter `v` pour le thread, ce qui signifiait que le thread principal pouvait théoriquement invalider la référence du thread lancé. En disant à Rust de transférer la possession de `v` au thread lancé, nous garantissons à Rust que le thread principal n'utilisera plus `v`. Si nous modifions l'exemple concernant l'utilisation d'une fermeture de la même manière, nous violons alors les règles de possession lorsque nous essayons d'utiliser `v` dans le thread principal. Le mot-clé `move` remplace le comportement par défaut conservateur de Rust d'emprunter ; il ne nous permet pas de violer les règles de possession.

Avec une compréhension de base des threads et de l'API des threads, examinons ce que nous pouvons _faire_ avec les threads.

Vous pouvez vous référer au chapitre suivant du livre de la programmation en Rust :
[Utilisation des threads pour exécuter du code simultanément](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)