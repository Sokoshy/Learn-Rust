### Mise à jour d'une table de hachage

Bien que le nombre de clés et de valeurs puisse croître, chaque clé ne peut avoir qu'une seule valeur associée à la fois. Lorsque vous souhaitez modifier les données dans une table de hachage, vous devez décider comment gérer le cas où une clé possède déjà une valeur assignée. Vous pourriez remplacer l'ancienne valeur par la nouvelle, en ignorant complètement l'ancienne valeur. Vous pourriez conserver l'ancienne valeur et ignorer la nouvelle valeur, en n'ajoutant la nouvelle valeur que si la clé *ne* possède pas déjà une valeur. Ou vous pourriez combiner l'ancienne valeur et la nouvelle valeur. Voyons comment faire chacun de ces cas !

#### Remplacer une valeur

Si nous insérons une clé et une valeur dans une table de hachage, puis insérons cette même clé avec une valeur différente, la valeur associée à cette clé sera remplacée. Même si le code ci-dessous appelle `insert` deux fois, la table de hachage ne contiendra qu'un seul couple clé/valeur car nous insérons la valeur pour la clé de l'équipe Bleue deux fois.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```

##### Remplacer une valeur stockée avec une clé particulière

Ce code affichera `{"Blue": 25}`. La valeur originale de `10` a été remplacée.

#### Insérer une valeur uniquement si la clé n'a pas de valeur

Il est courant de vérifier si une clé particulière a une valeur et, si elle ne l'a pas, d'insérer une valeur pour elle. Les tables de hachage ont une API spéciale pour cela appelée `entry` qui prend la clé que vous souhaitez vérifier en tant que paramètre. La valeur de retour de la méthode `entry` est une énumération appelée `Entry` qui représente une valeur qui pourrait exister ou non. Disons que nous voulons vérifier si la clé pour l'équipe Jaune a une valeur associée. Si ce n'est pas le cas, nous voulons insérer la valeur 50, et la même chose pour l'équipe Bleue. En utilisant l'API `entry`, le code ressemble à ceci :

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

##### Utiliser la méthode entry pour n'insérer que si la clé n'a pas déjà de valeur

La méthode `or_insert` sur `Entry` est définie pour retourner une référence mutable à la valeur pour la clé `Entry` correspondante si cette clé existe, et sinon, insère le paramètre comme nouvelle valeur pour cette clé et retourne une référence mutable à la nouvelle valeur. Cette technique est bien plus propre que d'écrire la logique nous-mêmes et, en plus, elle s'intègre mieux avec le vérificateur d'emprunt.

L'exécution du code ci-dessus affichera `{"Yellow": 50, "Blue": 25}`. Le premier appel à `entry` va insérer la clé pour l'équipe Jaune avec la valeur 50 car l'équipe Jaune n'a pas encore de valeur. Le second appel à `entry` ne changera pas la table de hachage car l'équipe Bleue a déjà la valeur 25.

#### Mettre à jour une valeur basée sur l'ancienne valeur

Un autre cas d'utilisation courant des tables de hachage est de rechercher la valeur d'une clé puis de la mettre à jour en fonction de l'ancienne valeur. Par exemple, l'exemple suivant montre un code qui compte combien de fois chaque mot apparaît dans un texte. Nous utilisons une table de hachage avec les mots comme clés et incrémentons la valeur pour suivre combien de fois nous avons vu ce mot. Si c'est la première fois que nous voyons un mot, nous insérons d'abord la valeur 0.

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

##### Compter les occurrences de mots à l'aide d'une table de hachage qui stocke les mots et les comptes

Ce code affichera `{"world": 2, "hello": 1, "wonderful": 1}`. La méthode `or_insert` retourne en fait une référence mutable (`&mut V`) à la valeur pour cette clé. Ici, nous stockons cette référence mutable dans la variable `count`, donc pour assigner à cette valeur, nous devons d'abord déréférencer `count` en utilisant l'astérisque (`*`). La référence mutable sort de la portée à la fin de la boucle `for`, ainsi toutes ces modifications sont sûres et autorisées par les règles d'emprunt.