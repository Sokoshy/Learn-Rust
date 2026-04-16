## Syntaxe des méthodes

Les *méthodes* sont similaires aux fonctions : elles sont déclarées avec le mot-clé `fn` et leur nom, elles peuvent avoir des paramètres et une valeur de retour, et elles contiennent du code qui est exécuté lorsqu'elles sont appelées depuis un autre endroit. Cependant, les méthodes diffèrent des fonctions en ce sens qu'elles sont définies dans le contexte d'une structure (ou d'une énumération ou d'un objet de trait, qui sont abordés dans le chapitre "Énumérations" et dans [le chapitre 17][ch17] du livre Rust, respectivement), et leur premier paramètre est toujours `self`, qui représente l'instance de la structure sur laquelle la méthode est appelée.

[ch17]: https://github.com/rust-lang/book/blob/master/src/ch17-00-oop.md