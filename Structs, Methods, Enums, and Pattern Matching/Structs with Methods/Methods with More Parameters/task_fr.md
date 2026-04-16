### Méthodes avec plusieurs paramètres

Pratiquons l'utilisation des méthodes en implémentant une deuxième méthode sur la structure `Rectangle`. Cette fois, nous voulons qu'une instance de `Rectangle` prenne une autre instance de `Rectangle` et retourne `true` si le second `Rectangle` peut s'insérer complètement dans `self`; sinon, elle doit retourner `false`. C'est-à-dire, nous voulons pouvoir écrire le programme montré ci-dessous, une fois que nous aurons défini la méthode `can_hold`.

```rust,ignore
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

#### Utilisation de la méthode `can_hold` encore non-écrite

Et le résultat attendu ressemblerait à ce qui suit, car les deux dimensions de `rect2` sont inférieures à celles de `rect1` mais `rect3` est plus large que `rect1` :

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Nous savons que nous voulons définir une méthode, elle sera donc dans le bloc `impl Rectangle`. Le nom de la méthode sera `can_hold`, et elle prendra un emprunt immuable d'un autre `Rectangle` comme paramètre. Nous pouvons déterminer quel sera le type du paramètre en regardant le code qui appelle la méthode : `rect1.can_hold(&rect2)` passe `&rect2`, qui est un emprunt immuable de `rect2`, une instance de `Rectangle`. Cela a du sens car nous avons seulement besoin de lire `rect2` (plutôt que d'écrire, ce qui nécessiterait un emprunt mutable), et nous voulons que `main` conserve la propriété de `rect2` pour que nous puissions l'utiliser de nouveau après avoir appelé la méthode `can_hold`. La valeur de retour de `can_hold` sera un booléen, et l'implémentation vérifiera si la largeur et la hauteur de `self` sont toutes deux supérieures à la largeur et à la hauteur de l'autre `Rectangle`, respectivement. Ajoutons la nouvelle méthode `can_hold` au bloc `impl` de la première liste de cette section, montrée ci-dessous.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### Implémentation de la méthode `can_hold` sur `Rectangle` qui prend une autre instance de `Rectangle` comme paramètre

Lorsque nous exécutons ce code avec la fonction `main` dans l'extrait de code précédent, nous obtiendrons notre résultat souhaité. Les méthodes peuvent prendre plusieurs paramètres que nous ajoutons à la signature après le paramètre `self`, et ces paramètres fonctionnent exactement comme les paramètres dans les fonctions.