### Élagage des durées de vie

Vous avez appris que chaque référence a une durée de vie et que vous devez spécifier des paramètres de durée de vie pour les fonctions ou les structures qui utilisent des références. Cependant, dans la section "Slices" de "Comprendre la propriété" dans le code "Améliorer la fonction `first_word` en utilisant un slice de chaîne pour le type du paramètre `s`", nous avions une fonction, montrée à nouveau ci-dessous, qui s'est compilée sans annotations de durée de vie.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

#### Une fonction définie dans "Slices" qui s'est compilée sans annotations de durée de vie, même si le paramètre et le type de retour sont des références

La raison pour laquelle cette fonction se compile sans annotations de durée de vie est historique : dans les premières versions (pré-1.0) de Rust, ce code n'aurait pas été compilé parce que chaque référence nécessitait une durée de vie explicite. À cette époque, la signature de la fonction aurait été écrite comme suit :

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Après avoir écrit beaucoup de code Rust, l'équipe de Rust a découvert que les programmeurs Rust entraient les mêmes annotations de durée de vie encore et encore dans certaines situations particulières. Ces situations étaient prévisibles et suivaient quelques motifs déterministes. Les développeurs ont programmé ces motifs dans le code du compilateur pour que le vérificateur de prêts puisse inférer les durées de vie dans ces situations et qu'il n'y ait pas besoin d'annotations explicites.

Cette partie de l'histoire de Rust est pertinente car il est possible que d'autres motifs déterministes apparaissent et soient ajoutés au compilateur. À l'avenir, encore moins d'annotations de durée de vie pourraient être nécessaires.

Les motifs programmés dans l'analyse des références de Rust sont appelés les *règles d'élagage des durées de vie*. Ce ne sont pas des règles à suivre pour les programmeurs ; ce sont un ensemble de cas particuliers que le compilateur considérera, et si votre code correspond à ces cas, vous n'avez pas besoin d'écrire les durées de vie explicitement.

Les règles d'élagage ne fournissent pas une inférence complète. Si Rust applique de manière déterministe les règles mais qu'il y a encore une ambiguïté quant aux durées de vie des références, le compilateur ne devinera pas quelles devraient être les durées de vie des références restantes. Dans ce cas, au lieu de deviner, le compilateur vous donnera une erreur que vous pouvez résoudre en ajoutant les annotations de durée de vie qui spécifient comment les références sont liées entre elles.

Les durées de vie sur les paramètres de fonction ou de méthode sont appelées *durées de vie d'entrée*, et les durées de vie sur les valeurs de retour sont appelées *durées de vie de sortie*.

Le compilateur utilise trois règles pour déterminer quelles durées de vie les références ont lorsqu'il n'y a pas d'annotations explicites. La première règle s'applique aux durées de vie d'entrée, et les deuxième et troisième règles s'appliquent aux durées de vie de sortie. Si le compilateur arrive à la fin des trois règles et qu'il y a encore des références pour lesquelles il ne peut pas déterminer les durées de vie, le compilateur s'arrêtera avec une erreur. Ces règles s'appliquent aux définitions de `fn` ainsi qu'aux blocs `impl`.

La première règle est que chaque paramètre qui est une référence obtient son propre paramètre de durée de vie. En d'autres termes, une fonction avec un paramètre obtient un paramètre de durée de vie : `fn foo<'a>(x: &'a i32)` ; une fonction avec deux paramètres obtient deux paramètres de durée de vie séparés : `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` ; et ainsi de suite.

La deuxième règle est que s'il y a exactement un paramètre de durée de vie d'entrée, cette durée de vie est assignée à tous les paramètres de durée de vie de sortie : `fn foo<'a>(x: &'a i32) -> &'a i32`.

La troisième règle est que s'il y a plusieurs paramètres de durée de vie d'entrée, mais que l'un d'entre eux est `&self` ou `&mut self` parce qu'il s'agit d'une méthode, la durée de vie de `self` est assignée à tous les paramètres de durée de vie de sortie. Cette troisième règle rend les méthodes beaucoup plus agréables à lire et à écrire car moins de symboles sont nécessaires.

Faisons semblant d'être le compilateur. Nous appliquerons ces règles pour déterminer quelles sont les durées de vie des références dans la signature de la fonction `first_word` dans l'énumération 10-26. La signature commence sans aucune durée de vie associée aux références :

```rust,ignore
fn first_word(s: &str) -> &str {
```

Puis le compilateur applique la première règle, qui spécifie que chaque paramètre obtient sa propre durée de vie. Nous l'appellerons `'a` comme d'habitude, donc maintenant la signature est la suivante :

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

La deuxième règle s'applique parce qu'il y a exactement une durée de vie d'entrée. La deuxième règle spécifie que la durée de vie de l'unique paramètre d'entrée est assignée à la durée de vie de sortie, donc la signature est maintenant la suivante :

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Maintenant, toutes les références dans cette signature de fonction ont des durées de vie, et le compilateur peut continuer son analyse sans nécessiter que le programmeur annote les durées de vie dans cette signature de fonction.

Examinons un autre exemple, cette fois en utilisant la fonction `longest`, qui n'avait pas de paramètres de durée de vie lorsque nous avons commencé à travailler avec elle dans l'énumération 10-21 :

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Appliquons la première règle : chaque paramètre obtient sa propre durée de vie. Cette fois, nous avons deux paramètres au lieu d'un, donc nous avons deux durées de vie :

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Vous pouvez constater que la deuxième règle ne s'applique pas car il y a plus d'une durée de vie d'entrée. La troisième règle ne s'applique pas non plus, car `longest` est une fonction plutôt qu'une méthode, donc aucun des paramètres n'est `self`. Après avoir passé en revue les trois règles, nous n'avons toujours pas déterminé quelle est la durée de vie du type de retour. C'est pourquoi nous avons obtenu une erreur en essayant de compiler le code dans l'énumération où nous avons d'abord essayé d'implémenter la fonction `longest` : le compilateur a passé en revue les règles d'élagage des durées de vie mais n'a toujours pas pu déterminer toutes les durées de vie des références dans la signature.

Parce que la troisième règle ne s'applique vraiment que dans les signatures de méthode, nous examinerons ensuite les durées de vie dans ce contexte pour voir pourquoi la troisième règle signifie que nous n'avons pas besoin d'annoter les durées de vie dans les signatures de méthode très souvent.