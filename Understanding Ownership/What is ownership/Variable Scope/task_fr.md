### Portée des variables

Nous avons déjà parcouru un exemple d'un programme en Rust au chapitre 2. Maintenant que nous avons dépassé la syntaxe de base, nous n'inclurons pas tout le code `fn main() {` dans les exemples, donc si vous suivez, vous devrez placer les exemples suivants à l'intérieur d'une fonction `main` manuellement. Ainsi, nos exemples seront un peu plus concis, nous permettant de nous concentrer sur les détails réels plutôt que sur le code standard.

Comme premier exemple de propriété, nous allons examiner la _portée_ de quelques variables. Une portée est l'intervalle dans un programme pour lequel un élément est valide. Disons que nous avons une variable qui ressemble à ceci :

```rust
let s = "hello"
```

La variable `s` fait référence à un littéral de chaîne, où la valeur de la chaîne est codée en dur dans le texte de notre programme. La variable est valide à partir du point où elle est déclarée jusqu'à la fin de la _portée_ actuelle. Le fragment de code ci-dessous comporte des commentaires annotant où la variable `s` est valide.

```rust
{                      // s n'est pas valide ici, elle n'est pas encore déclarée
let s = "hello";   // s est valide à partir de ce point

// faire des opérations avec s
}                      // cette portée est maintenant terminée, et s n'est plus valide
```

##### Une variable et la portée dans laquelle elle est valide

En d'autres termes, il y a deux moments importants ici :

*   Quand `s` entre en _portée_, elle est valide.
*   Elle reste valide jusqu'à ce qu'elle sorte de _portée_.

À ce stade, la relation entre les portées et les moments où les variables sont valides est similaire à celle dans d'autres langages de programmation. Nous allons maintenant approfondir cette compréhension en introduisant le type `String`.