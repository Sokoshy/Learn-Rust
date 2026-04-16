### Lecture des éléments des vecteurs

Maintenant que vous savez comment créer, mettre à jour et détruire des vecteurs, savoir comment lire leur contenu est une bonne étape suivante. Il existe deux manières de référencer une valeur stockée dans un vecteur. Dans les exemples, nous avons annoté les types des valeurs retournées par ces fonctions pour plus de clarté.

Le code ci-dessous montre les deux méthodes pour accéder à une valeur dans un vecteur, soit avec la syntaxe d'indexation, soit avec la méthode `get`.

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Le troisième élément est {}", third);

    match v.get(2) {
        Some(third) => println!("Le troisième élément est {}", third),
        None => println!("Il n'y a pas de troisième élément."),
    }
```

#### Utiliser la syntaxe d'indexation ou la méthode get pour accéder à un élément d'un vecteur

Notez deux détails ici. Premièrement, nous utilisons la valeur d'index de `2` pour obtenir le troisième élément : les vecteurs sont indexés par numéro, en commençant à zéro. Deuxièmement, les deux façons d'obtenir le troisième élément sont d'utiliser `&` et `[]`, ce qui nous donne une référence, ou d'utiliser la méthode `get` avec l'index passé en argument, ce qui nous donne un `Option<&T>`.

Rust a deux façons de référencer un élément afin que vous puissiez choisir comment le programme se comporte lorsque vous essayez d'utiliser une valeur d'index pour laquelle le vecteur n'a pas d'élément. À titre d'exemple, voyons ce que fera un programme s'il a un vecteur qui contient cinq éléments et qu'il essaie ensuite d'accéder à un élément à l'index 100, comme montré ci-dessous.

```rust,should_panic,panics
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
```

#### Tentative d'accès à l'élément à l'index 100 dans un vecteur contenant cinq éléments

Lorsque nous exécutons ce code, la première méthode `[]` fera paniquer le programme car elle référence un élément inexistant. Cette méthode est à utiliser si vous souhaitez que votre programme plante s'il y a une tentative d'accès à un élément au-delà de la fin du vecteur.

Lorsque la méthode `get` reçoit un index qui est hors du vecteur, elle retourne `None` sans provoquer de panique. Vous utiliseriez cette méthode si l'accès à un élément au-delà de l'intervalle du vecteur se produit occasionnellement dans des circonstances normales. Votre code aura alors une logique pour gérer soit `Some(&element)`, soit `None`, comme expliqué dans le chapitre "Enums". Par exemple, l'index pourrait provenir d'une personne entrant un chiffre. Si elle entre accidentellement un nombre trop grand et que le programme obtient une valeur `None`, vous pourriez lui indiquer combien d'éléments se trouvent dans le vecteur actuel et lui donner une autre chance d'entrer une valeur valide. Ce serait plus convivial que de faire planter le programme à cause d'une faute de frappe !