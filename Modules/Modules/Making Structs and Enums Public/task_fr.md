### Rendre les structures et les énumérations publiques

Nous pouvons également utiliser `pub` pour désigner les structures et les énumérations comme publiques, mais il y a quelques détails supplémentaires. Si nous utilisons `pub` avant une définition de structure, nous rendons la structure publique, mais ses champs restent privés. Nous pouvons rendre chaque champ public ou non au cas par cas. Dans l'exemple ci-dessous, nous avons défini une structure `back_of_house::Breakfast` publique avec un champ `toast` public mais un champ `seasonal_fruit` privé. Cela modélise le cas dans un restaurant où le client peut choisir le type de pain qui accompagne un repas, mais le chef décide du fruit qui accompagne le repas en fonction de ce qui est de saison et en stock. Le fruit disponible change rapidement, donc les clients ne peuvent pas choisir le fruit ou même voir le fruit qu'ils recevront.

```rust
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // Commander un petit déjeuner en été avec du pain de seigle
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Changer d'avis sur le type de pain souhaité
        meal.toast = String::from("Wheat");
        println!("Je voudrais du pain {} s'il vous plaît", meal.toast);

        // La ligne suivante ne se compilera pas si nous la décommentons ; nous n'avons pas le droit
        // de voir ou de modifier le fruit de saison qui accompagne le repas
        // meal.seasonal_fruit = String::from("blueberries");
    }
```

##### Une structure avec des champs publics et des champs privés

Parce que le champ `toast` dans la structure `back_of_house::Breakfast` est public, dans `eat_at_restaurant` nous pouvons lire et écrire sur le champ `toast` en utilisant la notation par point. Remarquez que nous ne pouvons pas utiliser le champ `seasonal_fruit` dans `eat_at_restaurant` parce que `seasonal_fruit` est privé. Essayez de décommenter la ligne modifiant la valeur du champ `seasonal_fruit` pour voir quelle erreur vous obtenez !

Notez également que, parce que `back_of_house::Breakfast` a un champ privé, la structure doit fournir une fonction associée publique qui construit une instance de `Breakfast` (nous l'avons nommée `summer` ici). Si `Breakfast` n'avait pas une telle fonction, nous ne pourrions pas créer une instance de `Breakfast` dans `eat_at_restaurant` parce que nous ne pourrions pas définir la valeur du champ privé `seasonal_fruit` dans `eat_at_restaurant`.

En revanche, si nous rendons une énumération publique, toutes ses variantes sont alors publiques. Nous n'avons besoin que du `pub` avant le mot-clé `enum`, comme montré ci-dessous.

```rust
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
```

##### Désigner une énumération comme publique rend toutes ses variantes publiques

Parce que nous avons rendu l'énumération `Appetizer` publique, nous pouvons utiliser les variantes `Soup` et `Salad` dans `eat_at_restaurant`. Les énumérations ne sont pas très utiles à moins que leurs variantes soient publiques; il serait ennuyeux de devoir annoter toutes les variantes d'énumérations avec `pub` dans chaque cas, donc le comportement par défaut pour les variantes d'énumérations est d'être public. Les structures sont souvent utiles sans que leurs champs soient publics, ainsi les champs des structures suivent la règle générale selon laquelle tout est privé par défaut à moins d'être annoté avec `pub`.

Il y a une autre situation impliquant `pub` que nous n'avons pas couverte, à savoir notre dernière fonctionnalité du système de modules : le mot-clé `use`. Nous couvrirons d'abord `use` par lui-même, puis nous montrerons comment combiner `pub` et `use`.

_Vous pouvez vous référer au chapitre suivant dans le livre de programmation Rust : [Référencer un élément dans l'arborescence de modules](https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)_