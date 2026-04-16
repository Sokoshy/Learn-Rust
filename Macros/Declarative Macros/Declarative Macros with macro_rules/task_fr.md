### Macros déclaratifs avec `macro_rules!` pour la métaprogrammation générale

La forme la plus couramment utilisée des macros en Rust est celle des _macros déclaratifs_. Celles-ci sont également parfois appelées "macros par exemple", "macros `macro_rules!`" ou tout simplement "macros". Fondamentalement, les macros déclaratifs vous permettent d'écrire quelque chose de similaire à une expression `match` de Rust. Comme expliqué au chapitre 6, les expressions `match` sont des structures de contrôle qui prennent une expression, comparent la valeur résultante de l'expression à des motifs, puis exécutent le code associé au motif correspondant. Les macros comparent également une valeur à des motifs associés à un code particulier : dans cette situation, la valeur est le code source Rust littéral passé à la macro; les motifs sont comparés à la structure de ce code source; et le code associé à chaque motif, lorsqu'il est apparié, remplace le code passé à la macro. Tout cela se passe pendant la compilation.

Pour définir une macro, vous utilisez la construction `macro_rules!`. Explorons comment utiliser `macro_rules!` en regardant comment la macro `vec!` est définie. Le chapitre 8 a expliqué comment nous pouvons utiliser la macro `vec!` pour créer un nouveau vecteur avec des valeurs particulières. Par exemple, la macro suivante crée un nouveau vecteur contenant trois entiers :

```rust
    let v: Vec<u32> = vec![1, 2, 3];
```

Nous pourrions également utiliser la macro `vec!` pour créer un vecteur de deux entiers ou un vecteur de cinq tranches de chaînes. Nous ne pourrions pas utiliser une fonction pour faire la même chose car nous ne connaîtrions pas le nombre ou le type de valeurs à l'avance.

Le fragment de code ci-dessous montre une définition légèrement simplifiée de la macro `vec!`.

```rust
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
```

##### Une version simplifiée de la définition de la macro vec!

> Remarque : La définition réelle de la macro `vec!` dans la bibliothèque standard inclut du code pour préallouer la quantité correcte de mémoire à l'avance. Ce code est une optimisation que nous n'incluons pas ici pour rendre l'exemple plus simple.

L'annotation `#[macro_export]` indique que cette macro doit être rendue disponible dès que la crate dans laquelle elle est définie est mise en portée. Sans cette annotation, la macro ne peut pas être mise en portée.

Nous commençons ensuite la définition de la macro avec `macro_rules!` et le nom de la macro que nous définissons _sans_ le point d'exclamation. Le nom, dans ce cas `vec`, est suivi de crochets délimitant le corps de la définition de la macro.

La structure dans le corps de `vec!` est similaire à celle d'une expression `match`. Ici, nous avons un bras avec le motif `( $( $x:expr ),* )`, suivi de `=>` et du bloc de code associé à ce motif. Si le motif correspond, le bloc de code associé sera émis. Étant donné qu'il s'agit du seul motif dans cette macro, il n'y a qu'une seule façon valable de correspondre; tout autre motif entraînerait une erreur. Les macros plus complexes auront plus d'un bras.

La syntaxe de motif valide dans les définitions de macros est différente de la syntaxe de motif abordée au chapitre 18 car les motifs de macros sont comparés à la structure du code Rust plutôt qu'à des valeurs. Examinons ce que signifient les éléments de motif dans le fragment de code ci-dessus; pour la syntaxe complète des motifs de macros, voir [la référence](https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html).

Tout d'abord, un ensemble de parenthèses englobe l'ensemble du motif. Un signe dollar (`$`) est ensuite suivi d'un ensemble de parenthèses qui capture les valeurs correspondant au motif à l'intérieur des parenthèses pour être utilisées dans le code de remplacement. À l'intérieur de `$()` se trouve `$x:expr`, qui correspond à toute expression Rust et donne à l'expression le nom `$x`.

La virgule suivant `$()` indique qu'un caractère séparateur virgule littéral pourrait éventuellement apparaître après le code qui correspond au code dans `$()`. Le `*` spécifie que le motif correspond à zéro ou plus de ce qui précède le `*`.

Lorsque nous appelons cette macro avec `vec![1, 2, 3];`, le motif `$x` correspond trois fois avec les trois expressions `1`, `2` et `3`.

Examinons maintenant le motif dans le corps du code associé à ce bras : `temp_vec.push()` à l'intérieur de `$()*` est généré pour chaque partie qui correspond à `$()` dans le motif zéro ou plusieurs fois selon le nombre de fois que le motif correspond. Le `$x` est remplacé par chaque expression appariée. Lorsque nous appelons cette macro avec `vec![1, 2, 3];`, le code généré qui remplace cet appel de macro sera le suivant :

```rust
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
```

Nous avons défini une macro qui peut prendre n'importe quel nombre d'arguments de n'importe quel type et peut générer du code pour créer un vecteur contenant les éléments spécifiés.

Il existe quelques cas particuliers étranges avec `macro_rules!`. À l'avenir, Rust disposera d'un second type de macro déclaratif qui fonctionnera de manière similaire mais corrigera certains de ces cas particuliers. Après cette mise à jour, `macro_rules!` sera effectivement déprécié. Avec cela à l'esprit, ainsi que le fait que la plupart des programmeurs Rust _utiliseront_ des macros plus souvent qu'ils n'en _écriront_, nous ne discuterons plus de `macro_rules!`. Pour en savoir plus sur l'écriture de macros, consultez la documentation en ligne ou d'autres ressources, telles que [“The Little Book of Rust Macros”](https://danielkeep.github.io/tlborm/book/index.html).