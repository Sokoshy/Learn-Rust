### Modèles catch-all et le symbole de remplacement `_`

Avec les enums, nous pouvons également prendre des mesures spéciales pour quelques valeurs particulières, mais pour toutes les autres valeurs prendre une action par défaut. Imaginez que nous implémentons un jeu où, si vous obtenez un 3 en lançant un dé, votre joueur ne bouge pas, mais reçoit un nouveau chapeau élégant. Si vous obtenez un 7, votre joueur perd un chapeau élégant. Pour toutes les autres valeurs, votre joueur avance d'un nombre d'espaces équivalent sur le plateau de jeu. Voici un `match` qui implémente cette logique, avec le résultat du lancer de dé codé en dur plutôt qu'une valeur aléatoire, et toute autre logique représentée par des fonctions sans corps car les implémenter est hors de portée pour cet exemple :

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

Pour les deux premiers bras, les modèles sont les valeurs littérales 3 et 7. Pour le dernier bras qui couvre toutes les autres valeurs possibles, le modèle est la variable que nous avons choisi de nommer `other`. Le code qui s'exécute pour le bras `other` utilise la variable en la passant à la fonction `move_player`.

Ce code compile, même si nous n'avons pas listé toutes les valeurs possibles qu'un u8 peut avoir, car le dernier modèle correspondra à toutes les valeurs non spécifiquement listées. Ce modèle catch-all répond à l'exigence selon laquelle `match` doit être exhaustif. Notez que nous devons placer le bras catch-all en dernier parce que les modèles sont évalués dans l'ordre. Rust nous avertira si nous ajoutons des bras après un catch-all car ces bras ultérieurs ne correspondraient jamais !

Rust a également un modèle que nous pouvons utiliser lorsque nous ne voulons pas utiliser la valeur dans le modèle catch-all : `_`, qui est un modèle spécial qui correspond à toute valeur et ne se lie pas à cette valeur. Cela indique à Rust que nous n'allons pas utiliser la valeur, donc Rust ne nous avertira pas au sujet d'une variable non utilisée.

Changeons les règles du jeu pour que si vous obtenez autre chose qu'un 3 ou un 7, vous devez relancer. Nous n'avons pas besoin d'utiliser la valeur dans ce cas, donc nous pouvons changer notre code pour utiliser `_` au lieu de la variable nommée `other` :

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

Cet exemple répond également à l'exigence d'exhaustivité parce que nous ignorons explicitement toutes les autres valeurs dans le dernier bras ; nous n'avons rien oublié.

Si nous changeons encore les règles du jeu, de sorte que rien d'autre ne se passe pendant votre tour si vous obtenez autre chose qu'un 3 ou un 7, nous pouvons exprimer cela en utilisant la valeur unité (le type tuple vide que nous avons mentionné dans la section “Le type Tuple”) comme le code qui accompagne le bras `_` :

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

Ici, nous disons explicitement à Rust que nous n'allons pas utiliser d'autres valeurs qui ne correspondent pas à un modèle dans un bras précédent, et nous ne voulons pas exécuter de code dans ce cas.

Il y a plus à découvrir sur les modèles et la correspondance que nous couvrirons dans le [Chapitre 18][ch18-00-patterns] du Livre Rust. Pour l'instant, nous allons passer à la syntaxe if let, qui peut être utile dans des situations où l'expression match est un peu verbeuse.

[ch18-00-patterns]: https://github.com/rust-lang/book/blob/master/src/ch18-00-patterns.md