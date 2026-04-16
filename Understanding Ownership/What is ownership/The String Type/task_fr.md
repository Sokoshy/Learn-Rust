## Le type String

Pour illustrer les règles de possession, nous avons besoin d'un type de données plus complexe que ceux que nous avons couverts dans la section [“Data Types”](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#data-types) du chapitre "Concepts de programmation communs". Les types présentés précédemment sont tous stockés sur la pile et retirés de la pile lorsque leur portée se termine, mais nous voulons examiner les données stockées sur le tas et comprendre comment Rust sait quand nettoyer ces données.

Nous utiliserons `String` comme exemple ici et nous concentrerons sur les parties de `String` qui concernent la possession. Ces aspects s'appliquent également à d'autres types de données complexes, qu'ils soient fournis par la bibliothèque standard ou créés par vous. Nous parlerons de `String` de manière plus approfondie dans le chapitre "Collections communes".

Nous avons déjà vu les littéraux de chaîne, où une valeur de chaîne est codée en dur dans notre programme. Les littéraux de chaîne sont pratiques, mais ne conviennent pas à chaque situation où nous pourrions vouloir utiliser du texte. Une raison est qu'ils sont immuables. Une autre raison est que toutes les valeurs de chaîne ne peuvent pas être connues lorsque nous écrivons notre code : par exemple, que se passe-t-il si nous voulons prendre une entrée utilisateur et la stocker ? Pour ces situations, Rust propose un second type de chaîne, `String`. Ce type est alloué sur le tas et, en tant que tel, est capable de stocker une quantité de texte qui nous est inconnue au moment de la compilation. Vous pouvez créer une `String` à partir d'un littéral de chaîne en utilisant la fonction `from`, comme ceci :

```rust
let s = String::from("hello");
```

Le double deux-points (`::`) est un opérateur qui nous permet de nommer cette fonction `from` particulière sous le type `String` plutôt que d'utiliser un nom quelconque comme `string_from`. Nous discuterons davantage de cette syntaxe dans la section [“Method Syntax”](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#method-syntax) du chapitre 5 et lorsque nous parlerons de l'espace de noms avec les modules dans [“Paths for Referring to an Item in the Module Tree”](https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) du chapitre 7.

Ce type de chaîne _peut_ être modifié :

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() ajoute un littéral à un String

println!("{}", s); // Cela affichera `hello, world!`
```

Quelle est donc la différence ici ? Pourquoi `String` peut-il être modifié mais pas les littéraux ? La différence réside dans la gestion de la mémoire de ces deux types.

### Mémoire et allocation

Dans le cas d'un littéral de chaîne, nous connaissons le contenu au moment de la compilation, de sorte que le texte est codé en dur directement dans l'exécutable final. C'est pourquoi les littéraux de chaîne sont rapides et efficaces. Mais ces propriétés découlent uniquement de l'immuabilité du littéral de chaîne. Malheureusement, nous ne pouvons pas intégrer un bloc de mémoire dans le binaire pour chaque morceau de texte dont la taille est inconnue au moment de la compilation et dont la taille pourrait changer pendant l'exécution du programme.

Avec le type `String`, pour prendre en charge un morceau de texte modifiable et extensible, nous devons allouer une quantité de mémoire sur le tas, inconnue au moment de la compilation, pour contenir le contenu. Cela signifie :

*   La mémoire doit être demandée à l'allocateur de mémoire à l'exécution.
*   Nous avons besoin d'un moyen de restituer cette mémoire à l'allocateur lorsque nous avons terminé avec notre `String`.

Cette première partie est effectuée par nous : lorsque nous appelons `String::from`, son implémentation demande la mémoire dont elle a besoin. C'est assez universel dans les langages de programmation.

Cependant, la deuxième partie est différente. Dans les langages avec un _ramasse-miettes (GC)_, le GC suit et nettoie la mémoire qui n'est plus utilisée, et nous n'avons pas besoin d'y penser. Sans GC, c'est notre responsabilité d'identifier quand la mémoire n'est plus utilisée et d'appeler le code pour la restituer explicitement, tout comme nous l'avons fait pour la demander. Faire cela correctement a été historiquement un problème de programmation difficile. Si nous oublions, nous gaspillons de la mémoire. Si nous le faisons trop tôt, nous aurons une variable invalide. Si nous le faisons deux fois, c'est également un bogue. Nous devons associer exactement un `allocate` avec exactement un `free`.

Rust emprunte un chemin différent : la mémoire est automatiquement restituée une fois que la variable qui la possède sort de la portée. Voici une version de notre exemple de portée du fragment de code ci-dessus utilisant un `String` au lieu d'un littéral de chaîne :

```rust
{
let s = String::from("hello"); // s est valide à partir de ce point

// faire des choses avec s
}                                  // cette portée est maintenant terminée, et s
// n'est plus valide
```

Il y a un point naturel où nous pouvons restituer la mémoire dont notre `String` a besoin à l'allocateur : lorsque `s` sort de portée. Lorsqu'une variable sort de portée, Rust appelle une fonction spéciale pour nous. Cette fonction est appelée `drop`, et c'est là que l'auteur de `String` peut mettre le code pour restituer la mémoire. Rust appelle `drop` automatiquement à l'accolade fermante.

> Remarque : En C++, ce modèle de désallocation des ressources à la fin de la durée de vie d'un élément est parfois appelé _Acquisition des ressources est initialisation (RAII)_. La fonction `drop` dans Rust vous sera familière si vous avez utilisé des modèles RAII.

Ce modèle a un impact profond sur la manière dont le code Rust est écrit. Cela peut sembler simple pour le moment, mais le comportement du code peut être inattendu dans des situations plus compliquées lorsque nous voulons que plusieurs variables utilisent les données que nous avons allouées sur le tas. Explorons maintenant certaines de ces situations.