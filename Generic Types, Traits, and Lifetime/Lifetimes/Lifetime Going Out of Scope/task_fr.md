### Durée de vie : sortir de la portée

Ensuite, essayons un exemple qui montre que la durée de vie de la référence dans `result` doit être la plus courte des deux arguments. Nous déplacerons la déclaration de la variable `result` à l'extérieur du bloc interne, mais laisserons l'affectation de la valeur à la variable `result` à l'intérieur du bloc avec `string2`. Puis, nous déplacerons le `println!` qui utilise `result` à l'extérieur du bloc interne, après la fin de celui-ci. Le code ci-dessous ne compilera pas.

```rust,ignore,does_not_compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

#### Tentative d'utilisation de `result` après que `string2` soit sorti de la portée

Quand nous essayons de compiler ce code, nous obtenons cette erreur :

```console
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

L'erreur montre que pour que `result` soit valide pour l'instruction `println!`, `string2` devrait être valide jusqu'à la fin du bloc externe. Rust le sait parce que nous avons annoté les durées de vie des paramètres de fonction et des valeurs de retour en utilisant le même paramètre de durée de vie `'a`.

En tant qu'humains, nous pouvons regarder ce code et voir que `string1` est plus long que `string2` et que, par conséquent, `result` contiendra une référence à `string1`. Comme `string1` n'est pas encore sorti de la portée, une référence à `string1` sera toujours valide pour l'instruction `println!`. Cependant, le compilateur ne peut pas voir que la référence est valide dans ce cas. Nous avons dit à Rust que la durée de vie de la référence renvoyée par la fonction `longest` est la même que la plus courte des durées de vie des références passées en argument. Par conséquent, le vérificateur d'emprunt interdit le code de la liste précédente comme pouvant potentiellement contenir une référence invalide.

Essayez de concevoir plus d'expériences qui varient les valeurs et les durées de vie des références passées à la fonction `longest` et comment la référence renvoyée est utilisée. Formulez des hypothèses sur le fait que vos expériences passeront ou non le vérificateur d'emprunt avant de compiler ; puis vérifiez si vous avez raison !