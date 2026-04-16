## Un exemple : gérer les rectangles

Pour comprendre quand nous pourrions vouloir utiliser des structures, écrivons un programme qui calcule la superficie d'un rectangle. Nous commencerons avec des variables simples, puis nous refactoriserons le programme jusqu'à ce que nous utilisions des structures à la place.

Créons un nouveau projet binaire avec Cargo appelé *rectangles* qui prendra la largeur et la hauteur d'un rectangle spécifiés en pixels et calculera l'aire du rectangle. La liste ci-dessous montre un court programme qui fait exactement cela dans le fichier *src/main.rs* de notre projet.

<span class="filename">Nom du fichier : src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "L'aire du rectangle est de {} pixels carrés.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

#### Calculer l'aire d'un rectangle spécifié par des variables séparées de largeur et de hauteur

Maintenant, exécutez ce programme en utilisant `cargo run` :

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/structs`
L'aire du rectangle est de 1500 pixels carrés.
```

Bien que le programme ci-dessus fonctionne et trouve l'aire du rectangle en appelant la fonction `area` avec chaque dimension, nous pouvons faire mieux. La largeur et la hauteur sont liées l'une à l'autre car ensemble, elles décrivent un rectangle.

Le problème de ce code est évident dans la signature de `area` :

```rust
fn area(width: u32, height: u32) -> u32 {
```

La fonction `area` est censée calculer l'aire d'un rectangle, mais la fonction que nous avons écrite a deux paramètres. Les paramètres sont liés, mais cela n'est exprimé nulle part dans notre programme. Il serait plus lisible et plus facile à gérer de regrouper la largeur et la hauteur. Nous avons déjà discuté d'une façon de le faire dans la section “Tuple” de la leçon "Concepts de programmation courants/Types de données" : en utilisant des tuples.