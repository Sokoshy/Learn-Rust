### L'énumération `Option` et ses avantages par rapport aux valeurs nulles

Dans la section précédente, nous avons vu comment l'énumération `IpAddr` nous permettait d'utiliser le système de types de Rust pour encoder plus d'informations que simplement les données dans notre programme. Cette section explore une étude de cas de `Option`, qui est une autre énumération définie par la bibliothèque standard. Le type `Option` est utilisé dans de nombreux endroits car il encode le scénario très courant dans lequel une valeur pourrait être quelque chose ou rien. Exprimer ce concept en termes du système de types signifie que le compilateur peut vérifier si vous avez traité tous les cas que vous devriez gérer ; cette fonctionnalité peut prévenir des bugs extrêmement courants dans d'autres langages de programmation.

La conception d'un langage de programmation est souvent pensée en termes des fonctionnalités que vous incluez, mais les fonctionnalités que vous excluez sont également importantes. Rust n'a pas la fonctionnalité de null que de nombreux autres langages ont. *Null* est une valeur qui signifie qu'il n'y a pas de valeur. Dans les langages avec null, les variables peuvent toujours être dans l'un de deux états : null ou non nul.

Dans sa présentation de 2009 “Null References: The Billion Dollar Mistake”, Tony Hoare, l'inventeur de null, a dit ceci :

> Je l'appelle mon erreur à un milliard de dollars. À cette époque, je concevais le premier système de types complet pour des références dans un langage orienté objet. Mon objectif était de garantir que toute utilisation de références serait absolument sûre, avec une vérification effectuée automatiquement par le compilateur. Mais je n'ai pas pu résister à la tentation d'y inclure une référence null, simplement parce que c'était si facile à implémenter. Cela a conduit à d'innombrables erreurs, vulnérabilités et plantages de systèmes, qui ont probablement causé un milliard de dollars de douleur et de dégâts au cours des quarante dernières années.

Le problème avec les valeurs nulles est que si vous essayez d'utiliser une valeur null comme une valeur non nulle, vous obtiendrez une erreur de quelque sorte. Parce que cette propriété de null ou non null est omniprésente, il est extrêmement facile de faire ce genre d'erreur.

Cependant, le concept que null essaie d'exprimer est toujours utile : null est une valeur qui est actuellement invalide ou absente pour une raison quelconque.

Le problème n'est pas vraiment avec le concept mais avec l'implémentation particulière. Ainsi, Rust n'a pas de null, mais il a une énumération qui peut encoder le concept d'une valeur présente ou absente. Cette énumération est `Option<T>`, et elle est [définie par la bibliothèque standard][option] comme suit :

[option]: https://doc.rust-lang.org/std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

L'énumération `Option<T>` est tellement utile qu'elle est même incluse dans le prélude ; vous n'avez pas besoin de l'importer explicitement. De plus, ses variantes aussi : vous pouvez utiliser `Some` et `None` directement sans le préfixe `Option::`. L'énumération `Option<T>` est toujours une énumération régulière, et `Some(T)` et `None` sont toujours des variantes de type `Option<T>`.

La syntaxe `<T>` est une fonctionnalité de Rust que nous n'avons pas encore abordée. C'est un paramètre de type générique, et nous couvrirons les génériques plus en détail dans le chapitre "Types Génériques, Traits et Durée de Vie". Pour l'instant, tout ce que vous devez savoir, c'est que `<T>` signifie que la variante `Some` de l'énumération `Option` peut contenir une donnée de n'importe quel type. Voici quelques exemples d'utilisation des valeurs `Option` pour contenir des types numériques et des types de chaînes de caractères :

```rust
let some_number = Some(5);
let some_string = Some("une chaîne");

let absent_number: Option<i32> = None;
```

Si nous utilisons `None` plutôt que `Some`, nous devons indiquer à Rust quel type de `Option<T>` nous avons car le compilateur ne peut pas déduire le type que la variante `Some` contiendra en regardant seulement une valeur `None`.

Quand nous avons une valeur `Some`, nous savons qu'une valeur est présente et la valeur est contenue dans le `Some`. Quand nous avons une valeur `None`, en quelque sorte, cela signifie la même chose que null : nous n'avons pas de valeur valide. Alors, pourquoi avoir `Option<T>` est-il meilleur que d'avoir null ?

En bref, parce que `Option<T>` et `T` (où `T` peut être n'importe quel type) sont des types différents, le compilateur ne nous laissera pas utiliser une valeur `Option<T>` comme si elle était définitivement une valeur valide. Par exemple, ce code ne se compilera pas car il essaie d'ajouter un `i8` à un `Option<i8>` :

```rust,ignore,does_not_compile
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Si nous exécutons ce code, nous obtenons un message d'erreur comme celui-ci :

```console
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ pas d'implémentation pour `i8 + Option<i8>`
  |
  = help: le trait `Add<Option<i8>>` n'est pas implémenté pour `i8`
  ```

Intense ! En fait, ce message d'erreur signifie que Rust ne sait pas comment ajouter un `i8` et un `Option<i8>` parce que ce sont des types différents. Quand nous avons une valeur d'un type comme `i8` en Rust, le compilateur s'assurera que nous avons toujours une valeur valide. Nous pouvons avancer en toute confiance sans avoir à vérifier si elle est null avant d'utiliser cette valeur. Seulement quand nous avons un `Option<i8>` (ou quel que soit le type de valeur avec lequel nous travaillons) devons-nous nous soucier de ne pas avoir une valeur, et le compilateur s'assurera que nous gérons ce cas avant d'utiliser la valeur.

En d'autres termes, vous devez convertir un `Option<T>` en un `T` avant de pouvoir effectuer des opérations `T` avec lui. En général, cela aide à attraper l'un des problèmes les plus courants avec null : supposer que quelque chose n'est pas null alors que c'est le cas.

Ne pas avoir à s'inquiéter de supposer incorrectement une valeur non null vous aide à être plus confiant dans votre code. Pour avoir une valeur qui peut éventuellement être null, vous devez explicitement choisir en rendant le type de cette valeur `Option<T>`. Ensuite, lorsque vous utilisez cette valeur, vous êtes tenu de gérer explicitement le cas où la valeur est null. Partout où une valeur a un type qui n'est pas un `Option<T>`, vous *pouvez* supposer en toute sécurité que la valeur n'est pas null. Ce fut une décision de conception délibérée de Rust pour limiter la présence envahissante de null et accroître la sécurité du code Rust.

Alors, comment obtenir la valeur `T` d'une variante `Some` lorsque vous avez une valeur de type `Option<T>` pour pouvoir utiliser cette valeur ? L'énumération `Option<T>` a un grand nombre de méthodes qui sont utiles dans une variété de situations ; vous pouvez les consulter dans [sa documentation][docs]. Se familiariser avec les méthodes sur `Option<T>` sera extrêmement utile dans votre parcours avec Rust.

[docs]: https://doc.rust-lang.org/std/option/enum.Option.html

En général, pour utiliser une valeur `Option<T>`, vous voulez avoir du code qui gérera chaque variante. Vous voulez un code qui s'exécutera uniquement lorsque vous avez une valeur `Some(T)`, et ce code est autorisé à utiliser le `T` interne. Vous voulez un autre code pour s'exécuter si vous avez une valeur `None`, et ce code n'a pas de valeur `T` disponible. L'expression `match` est une construction de flux de contrôle qui fait précisément cela lorsqu'elle est utilisée avec des énumérations : elle exécutera un code différent selon la variante de l'énumération qu'elle contient, et ce code peut utiliser les données à l'intérieur de la valeur correspondante.