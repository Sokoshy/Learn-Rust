### Référence vs `push`

Quand le programme a une référence valide, le vérificateur d'emprunts applique les règles de
propriété et d'emprunt (expliquées dans "Comprendre la propriété") pour s'assurer que cette référence
et toutes les autres références au contenu du vecteur restent valides. Rappelez-vous la
règle qui stipule que vous ne pouvez pas avoir de références mutables et immuables dans le même
portée. Cette règle s'applique dans le code ci-dessous, où nous détenons une référence immuable à
le premier élément d'un vecteur et essayons d'ajouter un élément à la fin, ce qui ne fonctionnera pas
si nous essayons également de référencer cet élément plus tard dans la fonction :

```rust,ignore,does_not_compile
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("Le premier élément est : {}", first);
```

#### Tenter d'ajouter un élément à un vecteur tout en tenant une référence à un élément

Compiler ce code entraînera cette erreur :

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - un emprunt immuable a lieu ici
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ un emprunt mutable a lieu ici
7 | 
8 |     println!("Le premier élément est : {}", first);
  |                                          ----- l'emprunt immuable est utilisé ici plus tard
```

Le code ci-dessus pourrait sembler fonctionner : pourquoi une référence
au premier élément devrait-elle se soucier de ce qui change à la fin du vecteur ? Cette
erreur est due à la manière dont fonctionnent les vecteurs : ajouter un nouvel élément à la fin du
vecteur pourrait nécessiter l'allocation de nouvelle mémoire et la copie des anciens éléments dans le
nouvel espace, s'il n'y a pas assez de place pour mettre tous les éléments côte à côte à l'endroit où
le vecteur se trouve actuellement. Dans ce cas, la référence au premier
élément pointerait vers de la mémoire désallouée. Les règles d'emprunt empêchent
les programmes de se retrouver dans cette situation.