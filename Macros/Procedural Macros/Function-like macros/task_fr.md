### Macros similaires à des fonctions

Les macros similaires à des fonctions définissent des macros qui ressemblent à des appels de fonctions. De manière similaire aux macros `macro_rules!`, elles sont plus flexibles que les fonctions ; par exemple, elles peuvent prendre un nombre d'arguments inconnu. Cependant, les macros `macro_rules!` ne peuvent être définies qu'en utilisant la syntaxe de type match que nous avons abordée dans la section [“Macros déclaratives avec `macro_rules!` pour la métaprogrammation générale”](https://doc.rust-lang.org/stable/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming). Les macros similaires à des fonctions prennent un paramètre `TokenStream` et leur définition manipule ce `TokenStream` en utilisant du code Rust, comme le font les deux autres types de macros procédurales. Un exemple de macro similaire à une fonction est une macro `sql!` qui pourrait être appelée ainsi :

```rust
    let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Cette macro analyserait l'instruction SQL à l'intérieur et vérifierait qu'elle est syntaxiquement correcte, ce qui est un traitement bien plus complexe qu'une macro `macro_rules!` ne peut faire. La macro `sql!` serait définie de la manière suivante :

```rust
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
```

Cette définition est similaire à la signature de la macro de dérivation personnalisée : nous recevons les jetons qui se trouvent à l'intérieur des parenthèses et renvoyons le code que nous voulons générer.