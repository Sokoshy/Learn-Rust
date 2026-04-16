### Annotations de durée de vie dans les définitions de struct

Jusqu'à présent, nous avons uniquement défini des structs pour contenir des types possédés. Il est possible que des structs contiennent des références, mais dans ce cas, nous devrions ajouter une annotation de durée de vie à chaque référence dans la définition du struct. L'exemple ci-dessous présente un struct nommé `ImportantExcerpt` qui contient une tranche de chaîne de caractères.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### Un struct qui contient une référence, donc sa définition nécessite une annotation de durée de vie

Ce struct a un champ, `part`, qui contient une tranche de chaîne de caractères, c'est-à-dire une référence. Comme pour les types de données génériques, nous déclarons le nom du paramètre générique de durée de vie à l'intérieur des crochets après le nom du struct afin de pouvoir utiliser le paramètre de durée de vie dans le corps de la définition du struct. Cette annotation signifie qu'une instance de `ImportantExcerpt` ne peut pas survivre à la référence qu'elle contient dans son champ `part`.

La fonction `main` ici crée une instance du struct `ImportantExcerpt` qui contient une référence à la première phrase de la `String` possédée par la variable `novel`. Les données dans `novel` existent avant la création de l'instance de `ImportantExcerpt`. De plus, `novel` ne sort pas de portée avant que `ImportantExcerpt` ne sorte de portée, donc la référence dans l'instance de `ImportantExcerpt` est valide.