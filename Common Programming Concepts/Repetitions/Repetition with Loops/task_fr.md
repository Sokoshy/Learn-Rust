## Répétition avec les boucles

Il est souvent utile d'exécuter un bloc de code plus d'une fois. Pour cette tâche, Rust propose plusieurs _boucles_. Une boucle exécute le code à l'intérieur de son corps jusqu'à la fin, puis recommence immédiatement depuis le début.

Rust a trois types de boucles : `loop`, `while` et `for`. Essayons chacune d'elles.

### Répéter du code avec loop

Le mot-clé `loop` indique à Rust d'exécuter un bloc de code encore et encore, indéfiniment ou jusqu'à ce que vous lui disiez explicitement de s'arrêter.

À titre d'exemple, modifiez le fichier src/main.rs pour qu'il ressemble à ceci :

```rust
fn main() {
     loop {
         println!("encore !");
     }
 }
```

Lorsque nous exécutons ce programme, nous verrons `encore !` s'afficher continuellement jusqu'à ce que nous arrêtions le programme manuellement. La plupart des terminaux prennent en charge un raccourci clavier, <span class="keystroke">ctrl-c</span>, pour interrompre un programme bloqué dans une boucle continue :

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
encore !
encore !
encore !
encore !
^Cencore !
```
Le symbole `^C` représente le moment où vous avez appuyé sur <span class="keystroke">ctrl-c</span>. Vous pouvez ou non voir le mot `encore !` affiché après le `^C`, selon l'endroit où le code était dans la boucle lorsqu'il a reçu le signal d'interruption.

Heureusement, Rust propose une autre manière, plus fiable, de sortir d'une boucle. Vous pouvez placer le mot-clé `break` à l'intérieur de la boucle pour indiquer au programme quand arrêter l'exécution de la boucle.

### Renvoyer des valeurs de boucles

Une des utilisations de `loop` est de réessayer une opération qui pourrait échouer, comme vérifier qu'un thread a terminé son travail. Cependant, vous pourriez avoir besoin de transmettre le résultat de cette opération au reste de votre code. Pour ce faire, vous pouvez ajouter la valeur que vous voulez renvoyer après l'expression `break` utilisée pour arrêter la boucle ; cette valeur sera renvoyée hors de la boucle afin que vous puissiez l'utiliser, comme montré ici :

```rust
fn main() {
    let mut compteur = 0;

    let resultat = loop {
        compteur += 1;

        if compteur == 10 {
            break compteur * 2;
        }
    };

    println!("Le résultat est {}", resultat);
}
```

Avant la boucle, nous déclarons une variable nommée `compteur` et l'initialisons à `0`. Ensuite, nous déclarons une variable nommée `resultat` pour contenir la valeur renvoyée par la boucle. À chaque itération de la boucle, nous ajoutons `1` à la variable `compteur`, puis nous vérifions si le compteur est égal à `10`. Lorsqu'il l'est, nous utilisons le mot-clé `break` avec la valeur `compteur * 2`. Après la boucle, nous utilisons un point-virgule pour terminer l'instruction qui assigne la valeur à `resultat`. Enfin, nous affichons la valeur dans `resultat`, qui dans ce cas est 20.

_Vous pouvez vous référer au chapitre suivant dans le livre Rust Programming Language : [Repetition with Loops](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#repetition-with-loops)_