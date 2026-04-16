### Capturer l'environnement avec les fermetures

Le premier aspect des fermetures que nous allons examiner est qu'elles peuvent capturer des valeurs de l'environnement dans lequel elles sont définies pour une utilisation ultérieure. Voici le scénario : une entreprise de t-shirts offre régulièrement un t-shirt gratuit à quelqu'un sur sa liste de diffusion. Les personnes sur la liste de diffusion peuvent ajouter facultativement leur couleur préférée à leur profil. Si la personne choisie pour recevoir le t-shirt gratuit a sa couleur préférée dans son profil, elle reçoit un t-shirt de cette couleur. Si la personne n'a pas spécifié de couleur préférée, elle reçoit la couleur dont l'entreprise a le plus en stock.

Il existe de nombreuses façons de mettre en œuvre cela. Pour cet exemple, nous allons utiliser un enum appelé `ShirtColor` qui a les variantes `Red` et `Blue`. L'inventaire de l'entreprise est représenté par une structure `Inventory` qui a un champ nommé `shirts` contenant un `Vec<ShirtColor>` représentant les t-shirts actuellement en stock. La méthode `shirt_giveaway` définie sur `Inventory` obtient la couleur de t-shirt préférée optionnelle de la personne recevant le t-shirt gratuit, et renvoie la couleur de t-shirt que la personne obtiendra. Ceci est montré dans la liste suivante :

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "L'utilisateur avec la préférence {:?} reçoit {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "L'utilisateur avec la préférence {:?} reçoit {:?}",
        user_pref2, giveaway2
    );
}
```

##### distribution de l'entreprise de t-shirts

Le `store` défini dans `main` a deux t-shirts bleus et un t-shirt rouge en stock. Ensuite, il appelle la méthode `giveaway` pour un utilisateur avec une préférence pour un t-shirt rouge et un utilisateur sans préférence. Exécuter ce code affiche :

```console
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
L'utilisateur avec la préférence Some(Red) reçoit Red
L'utilisateur avec la préférence None reçoit Blue
```

Encore une fois, ce code pourrait être mis en œuvre de nombreuses façons, mais celui-ci utilise des concepts que vous avez déjà appris, à l'exception du corps de la méthode `giveaway` qui utilise une fermeture. La méthode `giveaway` prend la préférence de l'utilisateur `Option<ShirtColor>` et appelle `unwrap_or_else` dessus. La [méthode `unwrap_or_else` sur `Option<T>`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else) est définie par la bibliothèque standard. Elle prend un argument : une fermeture sans arguments qui retourne une valeur `T` (le même type stocké dans la variante `Some` de l'`Option<T>`, dans ce cas, un `ShirtColor`). Si l'`Option<T>` est la variante `Some`, `unwrap_or_else` renvoie la valeur à l'intérieur du `Some`. Si l'`Option<T>` est la variante `None`, `unwrap_or_else` appelle la fermeture et renvoie la valeur retournée par la fermeture.

C'est intéressant car nous avons passé une fermeture qui appelle `self.most_stocked()` sur l'instance actuelle de `Inventory`. La bibliothèque standard n'avait pas besoin de connaître quoi que ce soit sur les types `Inventory` ou `ShirtColor` que nous avons définis, ni sur la logique que nous voulons utiliser dans ce scénario. La fermeture a capturé une référence immuable à l'instance `self` d'`Inventory` et l'a passée avec le code que nous avons spécifié à la méthode `unwrap_or_else`. Les fonctions ne sont pas capables de capturer leur environnement de cette manière.