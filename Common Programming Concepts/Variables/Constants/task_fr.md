### Constantes

Comme les variables immuables, les _constantes_ sont des valeurs liées à un nom et qui ne peuvent pas changer, mais il existe quelques différences entre les constantes et les variables.

Tout d'abord, vous ne pouvez pas utiliser `mut` avec les constantes. Les constantes ne sont pas seulement immuables par défaut, elles sont toujours immuables. Vous déclarez les constantes en utilisant le mot-clé `const` au lieu du mot-clé `let`, et le type de la valeur _doit_ être annoté. Nous allons aborder les types et les annotations de type dans la prochaine leçon, [Types de données de base](course://Common Programming Concepts/Basic Data Types), donc ne vous inquiétez pas des détails pour l'instant. Sachez simplement que vous devez toujours annoter le type.

Les constantes peuvent être déclarées dans n'importe quel scope, y compris le scope global, ce qui les rend utiles pour les valeurs que de nombreuses parties du code doivent connaître.

La dernière différence est que les constantes ne peuvent être définies que sur une expression constante, pas le résultat d'une valeur qui ne pourrait être calculée qu'à l'exécution.

Voici un exemple de déclaration de constante :

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Le nom de la constante est `THREE_HOURS_IN_SECONDS` et sa valeur est définie comme le résultat de la multiplication de 60 (le nombre de secondes dans une minute) par 60 (le nombre de minutes dans une heure) par 3 (le nombre d'heures que nous voulons compter dans ce programme). La convention de nommage de Rust pour les constantes est d'utiliser toutes les majuscules avec des underscores entre les mots. Le compilateur est capable d'évaluer un ensemble limité d'opérations à la compilation, ce qui nous permet de choisir d'écrire cette valeur de manière plus compréhensible et vérifiable, plutôt que de définir cette constante sur la valeur 10 800. Consultez la [section sur l'évaluation des constantes de la référence Rust](https://doc.rust-lang.org/stable/reference/const_eval.html) pour plus d'informations sur les opérations pouvant être utilisées lors de la déclaration de constantes.

Les constantes sont valables pendant toute la durée d'exécution d'un programme, dans le scope dans lequel elles ont été déclarées. Cette propriété rend les constantes utiles pour les valeurs de votre domaine d'application que plusieurs parties du programme pourraient devoir connaître, telles que le nombre maximum de points que tout joueur d'un jeu est autorisé à gagner ou la vitesse de la lumière.

Nommer les valeurs codées en dur utilisées dans tout votre programme sous forme de constantes est utile pour transmettre le sens de cette valeur aux futurs mainteneurs du code. Cela aide également à n'avoir qu'un seul endroit dans votre code où vous auriez besoin de changer si la valeur codée devait être mise à jour à l'avenir.

_Vous pouvez vous référer au chapitre suivant du livre « The Rust Programming Language » : [Constantes](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#constants)_