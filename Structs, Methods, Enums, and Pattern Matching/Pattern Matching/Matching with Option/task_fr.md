### Correspondance avec `Option<T>`

Dans la section précédente, nous voulions extraire la valeur intérieure `T` du cas `Some` lors de l'utilisation de `Option<T>` ; nous pouvons également gérer `Option<T>` en utilisant `match` comme nous l'avons fait avec l'énum `Coin` ! Au lieu de comparer des pièces, nous comparerons les variantes de `Option<T>`, mais le fonctionnement de l'expression `match` reste le même.

Disons que nous voulons écrire une fonction qui prend une `Option<i32>` et, s'il y a une valeur à l'intérieur, ajoute 1 à cette valeur. S'il n'y a pas de valeur à l'intérieur, la fonction doit retourner la valeur `None` et ne pas essayer d'effectuer d'opérations.

Cette fonction est très facile à écrire, grâce à `match`, et ressemblera au code ci-dessous.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### Une fonction qui utilise une expression `match` sur une `Option<i32>`

Examinons la première exécution de `plus_one` plus en détail. Lorsque nous appelons `plus_one(five)`, la variable `x` dans le corps de `plus_one` aura la valeur `Some(5)`. Nous comparons ensuite cela à chaque branche du match.

```rust,ignore
None => None,
```

La valeur `Some(5)` ne correspond pas au motif `None`, nous continuons donc à la branche suivante.

```rust,ignore
Some(i) => Some(i + 1),
```

Est-ce que `Some(5)` correspond à `Some(i)` ? Oui, c'est bien le cas ! Nous avons la même variante. Le `i` se lie à la valeur contenue dans `Some`, donc `i` prend la valeur `5`. Le code dans la branche du match est alors exécuté, nous ajoutons donc 1 à la valeur de `i` et créons une nouvelle valeur `Some` avec notre total `6` à l'intérieur.

Considérons maintenant le deuxième appel de `plus_one` dans l'encadré 6-5, où `x` est `None`. Nous entrons dans le `match` et comparons à la première branche.

```rust,ignore
None => None,
```

Ça correspond ! Il n'y a pas de valeur à laquelle ajouter, donc le programme s'arrête et retourne la valeur `None` du côté droit de `=>`. Comme la première branche correspondait, aucune autre branche n'est comparée.

La combinaison de `match` et d'énums est utile dans de nombreuses situations. Vous verrez souvent ce modèle dans le code Rust : `match` contre une énum, lier une variable aux données à l'intérieur, puis exécuter du code basé sur celles-ci. C'est un peu délicat au début, mais une fois que vous y êtes habitué, vous souhaiterez l'avoir dans tous les langages. C'est toujours un des favoris des utilisateurs.