### Réorganisation avec les structures : donner plus de sens

Nous utilisons des structures pour ajouter du sens en étiquetant les données. Nous pouvons transformer le tuple que nous utilisons en un type de données avec un nom pour l'ensemble ainsi que des noms pour les parties, comme indiqué ci-dessous.

<span class="filename">Nom de fichier : src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "La surface du rectangle est de {} pixels carrés.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### Définir une structure `Rectangle`

Ici, nous avons défini une structure et l'avons nommée `Rectangle`. À l'intérieur des accolades, nous avons défini les champs `width` et `height`, qui sont tous deux de type `u32`. Ensuite, dans `main`, nous avons créé une instance particulière de `Rectangle` qui a une largeur de 30 et une hauteur de 50.

Notre fonction `area` est maintenant définie avec un seul paramètre, que nous avons nommé `rectangle`, dont le type est un emprunt immuable d'une instance de la structure `Rectangle`. Comme mentionné dans le chapitre "Comprendre la propriété", nous voulons emprunter la structure plutôt qu'en prendre possession. De cette façon, `main` conserve sa possession et peut continuer à utiliser `rect1`, c'est la raison pour laquelle nous utilisons le `&` dans la signature de la fonction et lorsque nous appelons la fonction.

La fonction `area` accède aux champs `width` et `height` de l'instance `Rectangle`. Notre signature de fonction pour `area` exprime maintenant exactement notre intention : calculer la surface de `Rectangle` en utilisant ses champs `width` et `height`. Cela indique que la largeur et la hauteur sont liées entre elles et donne des noms descriptifs aux valeurs plutôt que d'utiliser les indices de tuple `0` et `1`. Cela améliore la clarté.