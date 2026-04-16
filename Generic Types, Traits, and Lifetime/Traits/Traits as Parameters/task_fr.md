### Traits en tant que paramètres

Maintenant que vous savez comment définir et implémenter des traits, nous pouvons explorer comment utiliser les traits pour définir des fonctions qui acceptent de nombreux types différents.

Par exemple, dans le deuxième exemple de cette section, nous avons implémenté le trait `Summary` sur les types `NewsArticle` et `Tweet`. Nous pouvons définir une fonction `notify` qui appelle la méthode `summarize` sur son paramètre `item`, qui est d'un type quelconque implémentant le trait `Summary`. Pour ce faire, nous pouvons utiliser la syntaxe `impl Trait`, comme ceci :

```rust,ignore
pub fn notify(item: &impl Summary) {
    println!("Dernières nouvelles ! {}", item.summarize());
}
```

Au lieu d'un type concret pour le paramètre `item`, nous spécifions le mot-clé `impl` et le nom du trait. Ce paramètre accepte tout type implémentant le trait spécifié. Dans le corps de `notify`, nous pouvons appeler n'importe quelle méthode sur `item` issue du trait `Summary`, comme `summarize`. Nous pouvons appeler `notify` et y passer une instance de `NewsArticle` ou `Tweet`. Un code qui appelle la fonction avec tout autre type, tel qu'un `String` ou un `i32`, ne compilera pas car ces types n'implémentent pas `Summary`.

#### Syntaxe des limites de trait

La syntaxe `impl Trait` fonctionne pour les cas simples mais n'est en réalité qu'un sucre syntaxique pour une forme plus longue, appelée une *limite de trait* ; cela ressemble à ceci :

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Dernières nouvelles ! {}", item.summarize());
}
```

Cette forme plus longue est équivalente à l'exemple dans la section précédente mais est plus verbeuse. Nous plaçons les limites de trait avec la déclaration du paramètre de type générique après un double-point et à l'intérieur de crochets angulaires.

La syntaxe `impl Trait` est pratique et permet un code plus concis dans les cas simples. La syntaxe de la limite de trait peut exprimer plus de complexité dans d'autres cas. Par exemple, nous pouvons avoir deux paramètres qui implémentent `Summary`. L'utilisation de la syntaxe `impl Trait` ressemble à ceci :

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Si nous voulions que cette fonction autorise `item1` et `item2` à avoir des types différents, l'utilisation de `impl Trait` serait appropriée (tant que les deux types implémentent `Summary`). Si nous voulions forcer les deux paramètres à avoir le même type, cela n'est possible à exprimer qu'en utilisant une limite de trait, comme ceci :

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

Le type générique `T` spécifié comme type des paramètres `item1` et `item2` contraint la fonction de sorte que le type concret de la valeur passée comme argument pour `item1` et `item2` doit être le même.

#### Spécifier plusieurs limites de trait avec la syntaxe `+`

Nous pouvons également spécifier plus d'une limite de trait. Supposons que nous voulions que `notify` utilise le formatage d'affichage sur `item` ainsi que la méthode `summarize` : nous spécifions dans la définition de `notify` que `item` doit implémenter à la fois `Display` et `Summary`. Nous pouvons le faire en utilisant la syntaxe `+` :

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

La syntaxe `+` est également valide avec les limites de trait sur les types génériques :

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

Avec les deux limites de trait spécifiées, le corps de `notify` peut appeler `summarize` et utiliser `{}` pour formater `item`.

#### Limites de trait plus claires avec les clauses `where`

Utiliser trop de limites de trait a ses inconvénients. Chaque générique a ses propres limites de trait, donc les fonctions avec plusieurs paramètres de type générique peuvent contenir beaucoup d'informations sur les limites de trait entre le nom de la fonction et sa liste de paramètres, rendant la signature de la fonction difficile à lire. Pour cette raison, Rust a une syntaxe alternative pour spécifier les limites de trait à l'intérieur d'une clause `where` après la signature de la fonction. Donc au lieu d'écrire ceci :

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

nous pouvons utiliser une clause `where`, comme ceci :

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

La signature de cette fonction est moins encombrée : le nom de la fonction, la liste de paramètres et le type de retour sont rapprochés, similaire à une fonction sans beaucoup de limites de trait.