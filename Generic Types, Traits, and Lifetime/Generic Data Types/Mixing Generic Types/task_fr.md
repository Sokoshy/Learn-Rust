### Mélange de types génériques

Les paramètres de type générique dans la définition d'une structure ne sont pas toujours les mêmes que ceux que vous utilisez dans les signatures des méthodes de cette structure. Par exemple, l'exemple ci-dessous définit la méthode `mixup` sur la structure `Point<T, U>` évoquée précédemment dans cette section. La méthode prend un autre `Point` en paramètre, qui peut avoir des types différents de ceux du `Point` `self` sur lequel nous appelons `mixup`. La méthode crée une nouvelle instance de `Point` avec la valeur de `x` provenant du `Point` `self` (de type `T`) et la valeur de `y` provenant du `Point` passé en paramètre (de type `W`).

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### Une méthode qui utilise des types génériques différents de ceux de la définition de sa structure

Dans `main`, nous avons défini un `Point` qui a un `i32` pour `x` (avec la valeur `5`) et un `f64` pour `y` (avec la valeur `10.4`). La variable `p2` est une structure `Point` qui a une tranche de chaîne pour `x` (avec la valeur `"Hello"`) et un `char` pour `y` (avec la valeur `c`). Appeler `mixup` sur `p1` avec l'argument `p2` nous donne `p3`, qui aura un `i32` pour `x`, car `x` provient de `p1`. La variable `p3` aura un `char` pour `y`, car `y` provient de `p2`. L'appel de la macro `println!` affichera `p3.x = 5, p3.y = c`.

L'objectif de cet exemple est de démontrer une situation où certains paramètres génériques sont déclarés avec `impl` et d'autres avec la définition de la méthode. Ici, les paramètres génériques `T` et `U` sont déclarés après `impl`, car ils correspondent à la définition de la structure. Les paramètres génériques `V` et `W` sont déclarés après `fn mixup`, car ils ne concernent que la méthode.