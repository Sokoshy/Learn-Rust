### Dans les définitions de méthode

Nous pouvons implémenter des méthodes sur les structs et les enums (comme nous l'avons fait dans le chapitre "Structs") et utiliser également des types génériques dans leurs définitions. Le extrait de code ci-dessous montre la struct `Point<T>` que nous avons définie plus haut avec une méthode nommée `x` implémentée dessus.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

#### Implémentation d'une méthode nommée `x` sur la struct `Point<T>` qui renverra une référence au champ `x` de type `T`.

Ici, nous avons défini une méthode nommée `x` sur `Point<T>` qui renvoie une référence aux données dans le champ `x`.

Notez que nous devons déclarer `T` juste après `impl` pour pouvoir l'utiliser pour spécifier que nous implémentons des méthodes sur le type `Point<T>`. En déclarant `T` comme un type générique après `impl`, Rust peut identifier que le type entre les chevrons dans `Point` est un type générique plutôt qu'un type concret.

Nous pourrions, par exemple, implémenter des méthodes uniquement sur des instances de `Point<f32>` plutôt que sur des instances de `Point<T>` avec un type générique quelconque. Ci-dessous, nous utilisons le type concret `f32`, ce qui signifie que nous ne déclarons aucun type après `impl`.

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

#### Un bloc `impl` qui s'applique uniquement à une struct avec un type concret particulier pour le paramètre de type générique `T`.

Ce code signifie que le type `Point<f32>` aura une méthode nommée `distance_from_origin` et que d'autres instances de `Point<T>` où `T` n'est pas de type `f32` n'auront pas cette méthode définie. La méthode mesure la distance de notre point par rapport au point aux coordonnées (0.0, 0.0) et utilise des opérations mathématiques qui ne sont disponibles que pour les types à virgule flottante.