### Utiliser les fermetures qui capturent leur environnement

Maintenant que nous avons introduit les itérateurs, nous pouvons démontrer une utilisation courante des fermetures qui capturent leur environnement en utilisant l'adaptateur d'itérateur `filter`. La méthode `filter` sur un itérateur prend une fermeture qui accepte chaque élément de l'itérateur et renvoie un booléen. Si la fermeture renvoie `true`, la valeur sera incluse dans l'itérateur produit par `filter`. Si la fermeture renvoie `false`, la valeur ne sera pas incluse dans l'itérateur résultant.

Dans l'extrait de code ci-dessous, nous utilisons `filter` avec une fermeture qui capture la variable `shoe_size` de son environnement pour itérer sur une collection d'instances de la structure `Shoe`. Elle retournera uniquement les chaussures qui sont de la taille spécifiée.

```rust
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 13, style: String::from("sandal") },
                Shoe { size: 10, style: String::from("boot") },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe { size: 10, style: String::from("sneaker") },
                    Shoe { size: 10, style: String::from("boot") },
                ]
            );
        }
    }
```

##### Utiliser la méthode filter avec une fermeture qui capture shoe_size

La fonction `shoes_in_my_size` prend possession d'un vecteur de chaussures et d'une taille de chaussure en tant que paramètres. Elle renvoie un vecteur contenant uniquement les chaussures de la taille spécifiée.

Dans le corps de `shoes_in_my_size`, nous appelons `into_iter` pour créer un itérateur qui prend possession du vecteur. Ensuite, nous appelons `filter` pour adapter cet itérateur en un nouvel itérateur qui ne contient que les éléments pour lesquels la fermeture renvoie `true`.

La fermeture capture le paramètre `shoe_size` de l'environnement et compare la valeur à la taille de chaque chaussure, ne conservant que les chaussures de la taille spécifiée. Enfin, l'appel à `collect` rassemble les valeurs retournées par l'itérateur adapté dans un vecteur qui est renvoyé par la fonction.

Le test montre que lorsque nous appelons `shoes_in_my_size`, nous obtenons seulement les chaussures qui ont la même taille que la valeur que nous avons spécifiée.