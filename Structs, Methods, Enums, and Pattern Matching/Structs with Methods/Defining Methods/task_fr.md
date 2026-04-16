### Définir des méthodes 

Modifions la fonction `area` qui prend une instance de `Rectangle` en paramètre pour en faire une méthode `area` définie sur la structure `Rectangle`, comme indiqué ci-dessous.

<span class="filename">Nom de fichier : src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "La superficie du rectangle est de {} pixels carrés.",
        rect1.area()
    );
}
```

#### Définir une méthode `area` sur la structure `Rectangle`

Pour définir la fonction dans le contexte de `Rectangle`, nous commençons un bloc `impl` (implémentation). Ensuite, nous déplaçons la fonction `area` dans les accolades de l'`impl` et remplaçons le premier (et dans ce cas, unique) paramètre par `self` dans la signature et partout dans le corps. Dans `main`, où nous avons appelé la fonction `area` et passé `rect1` en argument, nous pouvons plutôt utiliser la *syntaxe de méthode* pour appeler la méthode `area` sur notre instance de `Rectangle`. La syntaxe de méthode se place après une instance : nous ajoutons un point suivi du nom de la méthode, des parenthèses et des arguments éventuels.

Dans la signature pour `area`, nous utilisons `&self` au lieu de `rectangle: &Rectangle` car Rust sait que le type de `self` est `Rectangle` puisque cette méthode est dans le contexte de `impl Rectangle`. Notez que nous devons toujours utiliser `&` avant `self`, tout comme nous l'avons fait avec `&Rectangle`. Les méthodes peuvent prendre la possession de `self`, emprunter `self` de manière immuable comme nous l'avons fait ici, ou emprunter `self` de manière mutable, tout comme pour tout autre paramètre.

Nous avons choisi `&self` ici pour la même raison que nous avons utilisé `&Rectangle` dans la version fonction : nous ne voulons pas prendre possession, et nous voulons simplement lire les données dans la structure, pas les modifier. Si nous souhaitions modifier l'instance sur laquelle nous avons appelé la méthode dans le cadre de l'action de cette méthode, nous utiliserions `&mut self` comme premier paramètre. Avoir une méthode qui prend possession de l'instance en utilisant simplement `self` comme premier paramètre est rare ; cette technique est généralement utilisée lorsque la méthode transforme `self` en autre chose et que vous souhaitez empêcher l'appelant d'utiliser l'instance originale après la transformation.

Le principal avantage de l'utilisation des méthodes au lieu des fonctions, en plus de la syntaxe de méthode et du fait de ne pas répéter le type de `self` dans chaque signature de méthode, est l'organisation. Nous avons regroupé toutes les actions possibles avec une instance d'un type dans un seul bloc `impl` plutôt que d'obliger les futurs utilisateurs de notre code à rechercher les capacités de `Rectangle` à divers endroits dans la bibliothèque que nous fournissons.

> ### Où est l'opérateur `->` ?
>
> En C et C++, deux opérateurs différents sont utilisés pour appeler des méthodes : on utilise `.` si on appelle une méthode sur l'objet directement et `->` si on appelle la méthode sur un pointeur vers l'objet et qu'on doit d'abord déréférencer le pointeur. En d'autres termes, si `object` est un pointeur, `object->something()` est similaire à `(*object).something()`.
>
> Rust n’a pas d’équivalent pour l'opérateur `->`; à la place, Rust a une fonctionnalité appelée *référencement et déréférencement automatiques*. L'appel de méthodes est l'un des rares endroits en Rust où ce comportement existe.
>
> Voici comment cela fonctionne : lorsque vous appelez une méthode avec `object.something()`, Rust ajoute automatiquement `&`, `&mut`, ou `*` afin que `object` corresponde à la signature de la méthode. En d'autres termes, les éléments suivants sont identiques :
>
> ```rust
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> La première version est beaucoup plus claire. Ce comportement de référencement automatique fonctionne parce que les méthodes ont un récepteur clair — le type de `self`. Étant donné le récepteur et le nom d'une méthode, Rust peut déterminer de façon définitive si la méthode lit (`&self`), modifie (`&mut self`), ou consomme (`self`). Le fait que Rust rende l'emprunt implicite pour les récepteurs de méthodes est un élément clé pour rendre la possession plus ergonomique en pratique.