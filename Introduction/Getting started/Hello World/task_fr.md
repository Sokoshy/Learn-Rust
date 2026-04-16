## Bonjour, le monde !

Maintenant, écrivons votre premier programme en Rust. Traditionnellement, lorsque l'on apprend un nouveau langage, on commence par écrire un petit programme qui affiche le texte `Hello, world!` à l'écran, faisons donc la même chose ici !

Vous pouvez voir la fenêtre de l'**Éditeur** à gauche. C'est votre bac à sable et vous pouvez y tester votre code. Le fichier `main.rs` contient les lignes suivantes :

```rust
fn main() {
    // placez votre code ici pour le lancer
}
```
Remplaçons maintenant la ligne à l'intérieur de la fonction principale par le code :

```rust
    println!("Hello, world!");
```
##### Exemple : Un programme qui affiche Hello, world!

Pour exécuter ce code, cliquez sur l'icône **Exécuter** dans le coin supérieur gauche de l'éditeur. Cliquez sur le bouton **Vérifier** dans la fenêtre **Description de la tâche** pour exécuter les tests et vérifier la correction de la solution fournie.

Si `Hello, world!` s'est affiché, félicitations ! Vous avez officiellement écrit un programme Rust. Vous êtes donc un programmeur Rust—bienvenue !

### Anatomie d’un programme Rust

Examinons en détail ce qu'il s'est passé dans votre programme Hello, world! Voici la première pièce du puzzle :

```rust

fn main() {

}
```

Ces lignes définissent une fonction en Rust. La fonction `main` est spéciale : c'est toujours le premier code qui s'exécute dans chaque programme Rust exécutable. La première ligne déclare une fonction nommée `main` qui n'a pas de paramètres et ne retourne rien. S'il y avait des paramètres, ils seraient placés à l'intérieur des parenthèses, `()`.

De plus, notez que le corps de la fonction est encapsulé dans des accolades, `{}`. Rust exige ces guillemets autour de tous les corps de fonction. Il est de bon style de placer l'accolade ouvrante sur la même ligne que la déclaration de la fonction, en ajoutant un espace entre les deux.

Au moment de l'écriture de ce texte, un outil de formatage automatique appelé `rustfmt` est en cours de développement. Si vous souhaitez adhérer à un style standard à travers les projets Rust, `rustfmt` formatera votre code d'une manière particulière. L'équipe Rust prévoit d'inclure cet outil avec la distribution standard de Rust, comme `rustc`. Donc, selon le moment où vous lisez ce livre, il se peut qu'il soit déjà installé sur votre ordinateur ! Consultez la documentation en ligne pour plus de détails.

À l'intérieur de la fonction `main`, il y a le code suivant :

```rust
    println!("Hello, world!");
```

Cette ligne fait tout le travail dans ce petit programme : elle affiche du texte à l'écran. Quatre détails importants à noter ici. Premièrement, le style Rust est d'indenter avec quatre espaces, pas une tabulation.

Deuxièmement, println! appelle une macro Rust. Si elle appelait une fonction à la place, elle serait écrite comme `println` (sans le `!`). Nous discuterons des macros Rust plus en détail un peu plus tard. Pour l'instant, il suffit de savoir que l'utilisation de `!` signifie que vous appelez une macro au lieu d'une fonction normale.

Troisièmement, vous voyez la chaîne `"Hello, world!"`. Nous passons cette chaîne comme argument à `println!`, et la chaîne est affichée à l'écran.

Quatrièmement, nous terminons la ligne par un point-virgule (`;`), ce qui indique que cette expression est terminée et que la suivante est prête à commencer. La plupart des lignes de code Rust se terminent par un point-virgule.

_Vous pouvez vous référer au chapitre suivant dans le livre de la programmation en Rust : [Hello, World!](https://doc.rust-lang.org/stable/book/ch01-02-hello-world.html)_