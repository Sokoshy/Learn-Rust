### Durées de vie génériques dans les fonctions

Écrivons une fonction qui renvoie le plus long de deux segments de chaîne. Cette fonction prendra deux segments de chaîne et renverra un segment de chaîne. Après avoir implémenté la fonction `longest`, le code ci-dessous devrait afficher `La chaîne la plus longue est abcd`.

```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("La chaîne la plus longue est {}", result);
}
```

#### Une fonction `main` qui appelle la fonction `longest` pour trouver le plus long de deux segments de chaîne

Notez que nous voulons que la fonction prenne des segments de chaîne, qui sont des références, car nous ne voulons pas que la fonction `longest` prenne possession de ses paramètres. Référez-vous à la section “Segments de chaîne comme paramètres” dans "Comprendre la propriété" pour plus de discussions sur pourquoi les paramètres que nous utilisons dans l'extrait de code ci-dessus sont ceux que nous souhaitons.

Si nous essayons d'implémenter la fonction `longest` comme indiqué ci-dessous, elle ne se compilera pas.

```rust,ignore,does_not_compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### Une implémentation de la fonction `longest` qui renvoie le plus long de deux segments de chaîne mais ne se compile pas encore

Au lieu de cela, nous obtenons l'erreur suivante qui parle des durées de vie :

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```

Le texte d'aide révèle que le type de retour a besoin d'un paramètre de durée de vie générique, car Rust ne peut pas déterminer si la référence retournée fait référence à `x` ou `y`. En fait, nous ne le savons pas non plus, car le bloc `if` dans le corps de cette fonction renvoie une référence à `x` et le bloc `else` renvoie une référence à `y` !

Lorsque nous définissons cette fonction, nous ne connaissons pas les valeurs concrètes qui seront passées à cette fonction, donc nous ne savons pas si le cas `if` ou le cas `else` s'exécutera. Nous ne connaissons pas non plus les durées de vie concrètes des références qui seront passées, donc nous ne pouvons pas examiner les portées comme nous l'avons fait dans le deuxième et troisième extrait de code de cette section pour déterminer si la référence que nous retournons sera toujours valide. Le vérificateur d'emprunts ne peut pas non plus le déterminer, car il ne sait pas comment les durées de vie de `x` et `y` se rapportent à la durée de vie de la valeur de retour. Pour corriger cette erreur, nous allons ajouter des paramètres de durée de vie génériques qui définissent la relation entre les références afin que le vérificateur d'emprunts puisse effectuer son analyse.