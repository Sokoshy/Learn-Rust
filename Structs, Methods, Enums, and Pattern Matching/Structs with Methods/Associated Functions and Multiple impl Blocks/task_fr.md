### Fonctions associées

Une autre fonctionnalité utile des blocs `impl` est la possibilité de définir des fonctions à l'intérieur des blocs `impl` qui *ne* prennent pas `self` comme paramètre. Celles-ci sont appelées *fonctions associées* parce qu'elles sont associées à la structure. Ce sont toujours des fonctions, non des méthodes, car elles n'ont pas d'instance de la structure avec laquelle travailler. Vous avez déjà utilisé la fonction associée `String::from`.

Les fonctions associées sont souvent utilisées pour les constructeurs qui renverront une nouvelle instance de la structure. Par exemple, nous pourrions fournir une fonction associée qui aurait un paramètre de dimension et l'utiliserait comme largeur et hauteur, ce qui facilite la création d'un `Rectangle` carré au lieu de devoir spécifier la même valeur deux fois :

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

Pour appeler cette fonction associée, nous utilisons la syntaxe `::` avec le nom de la structure; `let sq = Rectangle::square(3);` en est un exemple. Cette fonction est dans l'espace de noms de la structure : la syntaxe `::` est utilisée à la fois pour les fonctions associées et pour les espaces de noms créés par les modules. Nous discuterons des modules dans le chapitre "Modules".

### Blocs `impl` multiples

Chaque structure est autorisée à avoir plusieurs blocs `impl`. Par exemple, le listing "Implémentation de la méthode `can_hold` sur `Rectangle` qui prend une autre instance de `Rectangle` comme paramètre" est équivalent au code montré ci-dessous, où chaque méthode se trouve dans son propre bloc `impl`.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### Réécriture du listing "Implémentation de la méthode `can_hold` sur `Rectangle` qui prend une autre instance de `Rectangle` comme paramètre" en utilisant plusieurs blocs `impl`

Il n'y a aucune raison de séparer ces méthodes en plusieurs blocs `impl` ici, mais c'est une syntaxe valide. Nous verrons un cas où plusieurs blocs `impl` sont utiles dans le chapitre "Types génériques, Traits et Durée de vie".