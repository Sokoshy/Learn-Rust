### Création d'une nouvelle chaîne

De nombreuses opérations disponibles avec `Vec<T>` sont également disponibles avec `String`, en commençant par la fonction `new` pour créer une chaîne, illustrée ci-dessous.
```rust
    let mut s = String::new();
```

##### Exemple de création d'une nouvelle chaîne vide

Cette ligne crée une nouvelle chaîne vide appelée `s`, dans laquelle nous pouvons ensuite charger des données. Souvent, nous avons des données initiales que nous voulons utiliser pour commencer la chaîne. Pour cela, nous utilisons la méthode `to_string`, disponible pour tout type qui implémente le trait `Display`, comme les littéraux de chaîne. La liste ci-dessous montre deux exemples.

```rust
    let data = "contient initial";

    let s = data.to_string();

    // la méthode fonctionne également directement sur un littéral :
    let s = "contient initial".to_string();
```

##### Exemple d'utilisation de la méthode to_string pour créer une chaîne à partir d'un littéral de chaîne

Ce code crée une chaîne contenant `contient initial`.

Nous pouvons également utiliser la fonction `String::from` pour créer une `String` à partir d'un littéral de chaîne. Le code ci-dessous est équivalent au code de la liste précédente qui utilise `to_string`.

```rust
    let s = String::from("contient initial");
```

##### Exemple d'utilisation de la fonction String::from pour créer une chaîne à partir d'un littéral de chaîne

Étant donné que les chaînes sont utilisées pour de nombreuses choses, nous pouvons utiliser de nombreuses API génériques différentes pour les chaînes, nous offrant ainsi de nombreuses options. Certaines peuvent sembler redondantes, mais elles ont toutes leur utilité ! Dans ce cas, `String::from` et `to_string` font la même chose, donc le choix relève du style.

N'oubliez pas que les chaînes sont encodées en UTF-8, de sorte que nous pouvons inclure n'importe quelles données correctement encodées en elles, comme illustré ci-dessous.

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

##### Exemple de stockage de salutations en différentes langues dans des chaînes

Toutes ces valeurs sont des `String` valides.