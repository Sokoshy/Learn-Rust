### Utiliser un enum pour stocker plusieurs types

Au début de ce chapitre, nous avons dit que les vecteurs ne peuvent stocker que des valeurs du même type. Cela peut être peu pratique ; il existe certainement des cas d'utilisation où il est nécessaire de stocker une liste d'éléments de types différents. Heureusement, les variantes d'un enum sont définies sous le même type enum, donc lorsque nous avons besoin de stocker des éléments de types différents dans un vecteur, nous pouvons définir et utiliser un enum !

Par exemple, supposons que nous voulons obtenir des valeurs d'une ligne dans un tableur où certaines des colonnes de la ligne contiennent des entiers, d'autres des nombres à virgule flottante, et d'autres des chaînes de caractères. Nous pouvons définir un enum dont les variantes contiendront les différents types de valeurs, et alors toutes les variantes de l'enum seront considérées du même type : celui de l'enum. Ensuite, nous pouvons créer un vecteur qui contient cet enum et ainsi, finalement, contenir différents types. Nous l'avons démontré ci-dessous.

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

#### Définir un enum pour stocker des valeurs de différents types dans un vecteur

Rust doit savoir quels types seront dans le vecteur au moment de la compilation pour savoir exactement combien de mémoire sur le tas sera nécessaire pour stocker chaque élément. Un avantage secondaire est que nous pouvons être explicites quant aux types autorisés dans ce vecteur. Si Rust permettait à un vecteur de contenir n'importe quel type, il pourrait y avoir une chance qu'un ou plusieurs des types provoquent des erreurs avec les opérations effectuées sur les éléments du vecteur. Utiliser un enum ainsi qu'une expression `match` signifie que Rust garantira au moment de la compilation que chaque cas possible est traité, comme discuté dans le chapitre "Enums".

Lorsque vous écrivez un programme, si vous ne savez pas l'ensemble exhaustif des types que le programme obtiendra au moment de l'exécution pour les stocker dans un vecteur, la technique de l'enum ne fonctionnera pas. À la place, vous pouvez utiliser un objet de trait, que nous couvrirons dans le chapitre 17 du [livre Rust][book].

Maintenant que nous avons discuté de certaines des façons les plus courantes d'utiliser des vecteurs, assurez-vous de consulter [la documentation de l'API][vec-api] pour toutes les nombreuses méthodes utiles définies sur `Vec<T>` par la bibliothèque standard. Par exemple, en plus de `push`, une méthode `pop` supprime et renvoie le dernier élément.