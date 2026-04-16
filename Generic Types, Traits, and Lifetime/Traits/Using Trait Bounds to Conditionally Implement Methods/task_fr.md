### Utilisation des limites de traits pour implémenter conditionnellement des méthodes

En utilisant une limite de trait avec un bloc `impl` qui utilise des paramètres de type génériques, nous pouvons implémenter des méthodes conditionnellement pour les types qui implémentent les traits spécifiés. Par exemple, le type `Pair<T>` dans l'exemple ci-dessous implémente toujours la fonction `new`. Mais `Pair<T>` n'implémente la méthode `cmp_display` que si son type interne `T` implémente le trait `PartialOrd` qui permet la comparaison *et* le trait `Display` qui permet l'affichage.

```rust,noplayground
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Le plus grand membre est x = {}", self.x);
        } else {
            println!("Le plus grand membre est y = {}", self.y);
        }
    }
}
```

#### implémenter conditionnellement des méthodes sur un type générique en fonction des limites de traits

Nous pouvons également implémenter conditionnellement un trait pour tout type qui implémente un autre trait. Les implémentations d'un trait pour tout type qui satisfait aux limites de traits sont appelées *implémentations génériques* et sont largement utilisées dans la bibliothèque standard de Rust. Par exemple, la bibliothèque standard implémente le trait `ToString` pour tout type qui implémente le trait `Display`. Le bloc `impl` dans la bibliothèque standard ressemble à ce code :

```rust,ignore
impl<T: Display> ToString for T {
    // --troncature--
}
```

Parce que la bibliothèque standard a cette implémentation générique, nous pouvons appeler la méthode `to_string` définie par le trait `ToString` sur tout type qui implémente le trait `Display`. Par exemple, nous pouvons convertir des entiers en leurs valeurs `String` correspondantes comme ceci parce que les entiers implémentent `Display` :

```rust
let s = 3.to_string();
```

Les implémentations génériques apparaissent dans la documentation du trait dans la section “Implementors”.

Les traits et les limites de traits nous permettent d'écrire du code qui utilise des paramètres de type génériques pour réduire la duplication mais aussi pour spécifier au compilateur que nous voulons que le type générique ait un comportement particulier. Le compilateur peut alors utiliser les informations de limite de trait pour vérifier que tous les types concrets utilisés avec notre code fournissent le comportement correct. Dans les langages typés dynamiquement, nous obtiendrions une erreur à l'exécution si nous appelions une méthode sur un type qui ne définissait pas la méthode. Mais Rust déplace ces erreurs au moment de la compilation, nous obligeant ainsi à corriger les problèmes avant même que notre code puisse être exécuté. De plus, nous n'avons pas à écrire de code qui vérifie le comportement à l'exécution car nous avons déjà vérifié au moment de la compilation. Cela améliore la performance sans avoir à abandonner la flexibilité des génériques.