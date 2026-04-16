## Manières dont les variables et les données interagissent : clone

Si nous _voulons_ copier en profondeur les données du tas de la `String`, et non seulement les données de la pile, nous pouvons utiliser une méthode courante appelée `clone`. Nous aborderons la syntaxe des méthodes dans le chapitre "Structures", mais comme les méthodes sont une caractéristique courante dans de nombreux langages de programmation, vous les avez probablement déjà rencontrées.

Voici un exemple de la méthode `clone` en action :

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

Cela fonctionne très bien et produit explicitement le comportement illustré dans la Figure 3, où les données du tas _sont_ effectivement copiées.

Quand vous voyez un appel à `clone`, vous savez qu’un certain code arbitraire est exécuté et que ce code peut être coûteux. C’est un indicateur visuel qu’il se passe quelque chose de particulier.

## Données uniquement sur la pile : copy

Il y a un autre détail que nous n’avons pas encore abordé. Ce code utilisant des entiers, dont une partie a été montrée dans l’extrait "Attribuer la valeur entière de la variable x à y", fonctionne et est valide :

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Mais ce code semble contredire ce que nous venons d’apprendre : nous n’avons pas d’appel à `clone`, mais `x` est toujours valide et n’a pas été déplacé vers `y`.

La raison est que les types comme les entiers, qui ont une taille connue à la compilation, sont entièrement stockés sur la pile, de sorte que les copies des valeurs réelles sont rapides à effectuer. Cela signifie qu’il n’y a aucune raison d’empêcher `x` d’être valide après avoir créé la variable `y`. En d’autres termes, il n’y a pas de différence entre la copie profonde et la copie superficielle ici, donc appeler `clone` ne ferait rien de différent de la copie superficielle habituelle et nous pouvons l’omettre.

Rust a une annotation spéciale appelée le trait `Copy` que nous pouvons placer sur des types comme les entiers qui sont stockés sur la pile (nous parlerons plus des traits au chapitre 10). Si un type implémente le trait `Copy`, une ancienne variable est toujours utilisable après l’affectation. Rust ne nous laisse pas annoter un type avec le trait `Copy` si le type, ou l’une de ses parties, a implémenté le trait `Drop`. Si le type nécessite quelque chose de spécial lorsque la valeur sort du champ d'application et que nous ajoutons l’annotation `Copy` à ce type, nous obtiendrons une erreur à la compilation. Pour savoir comment ajouter l’annotation `Copy` à votre type pour implémenter le trait, consultez [“Traits dérivables”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) dans l’annexe C.

Alors, quels types implémentent le trait `Copy` ? Vous pouvez consulter la documentation pour le type donné pour en être certain, mais en règle générale, tout groupe de valeurs scalaires simples peut implémenter `Copy`, et rien qui nécessite une allocation ou est une forme de ressource ne peut implémenter `Copy`. Voici quelques-uns des types qui implémentent `Copy` :

*   Tous les types entiers, comme `u32`.
*   Le type booléen, `bool`, avec les valeurs `true` et `false`.
*   Tous les types à virgule flottante, comme `f64`.
*   Le type caractère, `char`.
*   Les tuples, s'ils ne contiennent que des types qui implémentent également `Copy`. Par exemple, `(i32, i32)` implémente `Copy`, mais `(i32, String)` ne le fait pas.