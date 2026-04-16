### Réusinage avec des tuples

L'extrait de code ci-dessous montre une autre version de notre programme qui utilise des tuples.

<span class="filename">Nom de fichier : src/main.rs</span>

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "La surface du rectangle est de {} pixels carrés.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

#### Spécification de la largeur et de la hauteur du rectangle avec un tuple

D'une certaine manière, ce programme est meilleur. Les tuples nous permettent d'ajouter un peu de structure et nous passons maintenant un seul argument. Mais d'une autre manière, cette version est moins claire : les tuples ne nomment pas leurs éléments, donc notre calcul est devenu plus confus parce que nous devons indexer les parties du tuple.

Il n'y a pas d'importance si nous confondons largeur et hauteur pour le calcul de la surface, mais si nous voulons dessiner le rectangle à l'écran, cela serait important ! Nous devrions garder à l'esprit que `width` est l'indice `0` du tuple et `height` est l'indice `1`. Si quelqu'un d'autre travaillait sur ce code, il devrait également comprendre cela et le garder à l'esprit. Il serait facile d'oublier ou de confondre ces valeurs et de provoquer des erreurs parce que nous n'avons pas transmis la signification de nos données dans notre code.