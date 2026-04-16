## Façons dont les variables et les données interagissent : déplacement

Plusieurs variables peuvent interagir avec les mêmes données de différentes manières en Rust. Prenons un exemple en utilisant un entier dans l'extrait de code ci-dessous.

```rust
let x = 5;
let y = x;
```

##### Affectation de la valeur entière de la variable x à y

Nous pouvons probablement deviner ce qui se passe : « lier la valeur `5` à `x`; puis faire une copie de la valeur dans `x` et la lier à `y` ». Nous avons maintenant deux variables, `x` et `y`, et toutes deux valent `5`. C'est effectivement ce qui se passe, car les entiers sont des valeurs simples de taille fixe connue, et ces deux valeurs `5` sont poussées sur la pile.

Voyons maintenant la version `String` :

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Cela ressemble beaucoup au code précédent, donc nous pourrions supposer que son fonctionnement serait le même : c'est-à-dire que la seconde ligne ferait une copie de la valeur dans `s1` et la lierait à `s2`. Mais ce n'est pas tout à fait ce qui se passe.

Regardez la Figure 1 pour voir ce qui se passe sous le capot avec `String`. Une `String` est composée de trois parties, montrées à gauche : un pointeur vers la mémoire qui contient le contenu de la chaîne, une longueur et une capacité. Ce groupe de données est stocké sur la pile. À droite se trouve la mémoire dans le tas qui contient le contenu.

<img alt="String en mémoire" src="https://doc.rust-lang.org/stable/book/img/trpl04-01.svg" class="center" style="width: 50%;">

##### Figure 1 : Représentation en mémoire d'une String contenant la valeur "hello" liée à s1

La longueur est la quantité de mémoire, en octets, que le contenu de la `String` utilise actuellement. La capacité est le montant total de mémoire, en octets, que la `String` a reçu de l'allocateur. La différence entre la longueur et la capacité est importante, mais pas dans ce contexte, donc pour l'instant, il est acceptable d'ignorer la capacité.

Lorsque nous affectons `s1` à `s2`, les données de la `String` sont copiées, ce qui signifie que nous copions le pointeur, la longueur et la capacité qui sont sur la pile. Nous ne copions pas les données sur le tas auxquelles le pointeur fait référence. En d'autres termes, la représentation des données en mémoire ressemble à la Figure 2.

<img alt="s1 et s2 pointant vers la même valeur" src="https://doc.rust-lang.org/stable/book/img/trpl04-02.svg" class="center" style="width: 50%;">

##### Figure 2 : Représentation en mémoire de la variable s2 qui a une copie du pointeur, de la longueur et de la capacité de s1

La représentation ne ressemble _pas_ à la Figure 3, qui est ce à quoi ressemblerait la mémoire si Rust copiait également les données du tas. Si Rust faisait cela, l'opération `s2 = s1` pourrait être très coûteuse en termes de performances à l'exécution si les données du tas étaient volumineuses.

<img alt="s1 et s2 vers deux endroits" src="https://doc.rust-lang.org/stable/book/img/trpl04-03.svg" class="center" style="width: 50%;">

##### Figure 3 : Une autre possibilité de ce que s2 = s1 pourrait faire si Rust copiait également les données du tas

Nous avons dit plus tôt que lorsqu'une variable sort de portée, Rust appelle automatiquement la fonction `drop` et libère la mémoire du tas pour cette variable. Mais la Figure 2 montre les deux pointeurs de données pointant vers le même emplacement. C'est un problème : lorsque `s2` et `s1` sortent de portée, ils tenteront tous deux de libérer la même mémoire. C'est ce qu'on appelle une erreur de _double libération_ et c'est l'un des bogues de sécurité mémoire que nous avons mentionnés précédemment. Libérer la mémoire deux fois peut conduire à une corruption de la mémoire, ce qui peut potentiellement entraîner des vulnérabilités de sécurité.

Pour assurer la sécurité mémoire, il y a un détail supplémentaire concernant ce qui se passe dans cette situation en Rust. Au lieu d'essayer de copier la mémoire allouée, Rust considère `s1` comme n'étant plus valide et, par conséquent, Rust n'a pas besoin de libérer quoi que ce soit lorsque `s1` sort de portée. Regardez ce qui se passe lorsque vous essayez d'utiliser `s1` après que `s2` a été créé; cela ne fonctionnera pas :

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

Vous obtiendrez une erreur comme celle-ci parce que Rust vous empêche d'utiliser la référence invalidée :

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
```

Si vous avez entendu les termes _copie superficielle_ et _copie profonde_ en travaillant avec d'autres langages, le concept de copie du pointeur, de la longueur et de la capacité sans copier les données ressemble probablement à une copie superficielle. Mais parce que Rust invalide également la première variable, au lieu d'être appelée une copie superficielle, c'est connu sous le nom de _déplacement_. Dans cet exemple, nous dirions que `s1` a été _déplacé_ vers `s2`. Ainsi, ce qui se passe réellement est montré à la Figure 4.

<img alt="s1 déplacé vers s2" src="https://doc.rust-lang.org/stable/book/img/trpl04-04.svg" class="center" style="width: 50%;">

##### Figure 4 : Représentation en mémoire après l'invalidation de `s1`

Cela résout notre problème ! Avec seulement `s2` valide, lorsqu'il sort de portée, il libérera seul la mémoire, et nous avons terminé.

De plus, une décision de conception est sous-entendue ici : Rust ne créera jamais automatiquement de copies « profondes » de vos données. Par conséquent, toute copie _automatique_ peut être supposée peu coûteuse en termes de performances à l'exécution.