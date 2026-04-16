## Restriction des références mutables

Mais les références mutables ont une grande restriction : vous ne pouvez avoir qu'une seule référence mutable à une donnée particulière dans un contexte particulier. Ce code échouera :

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

Voici l'erreur :

```text
    erreur[E0499]: ne peut pas emprunter `s` comme mutable plus d'une fois à la fois
     --> src/main.rs:5:14
      |
    4 |     let r1 = &mut s;
      |              ------ le premier emprunt mutable a lieu ici
    5 |     let r2 = &mut s;
      |              ^^^^^^ le second emprunt mutable a lieu ici
    6 |
    7 |     println!("{}, {}", r1, r2);
      |                        -- le premier emprunt est utilisé ici plus tard
```

Cette restriction permet la mutation mais de manière très contrôlée. C'est quelque chose avec lequel les nouveaux développeurs Rust luttent car la plupart des langages vous permettent de muter quand vous le souhaitez.

L'avantage de cette restriction est que Rust peut prévenir les courses de données à la compilation. Une _course de données_ est similaire à une condition de compétition et se produit lorsque ces trois comportements surviennent :

*   Deux ou plusieurs pointeurs accèdent aux mêmes données au même moment.
*   Au moins un des pointeurs est utilisé pour écrire sur les données.
*   Il n'y a aucun mécanisme utilisé pour synchroniser l'accès aux données.

Les courses de données causent un comportement indéfini et peuvent être difficiles à diagnostiquer et à corriger lorsque vous essayez de les suivre en cours d'exécution ; Rust empêche ce problème de se produire car il ne permettra même pas de compiler du code contenant des courses de données !

Comme toujours, nous pouvons utiliser des accolades pour créer un nouveau contexte, autorisant plusieurs références mutables, mais pas des références _simultanées_ :

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 sort du contexte ici, donc nous pouvons créer une nouvelle référence sans problème.

    let r2 = &mut s;
```

Une règle similaire existe pour combiner des références mutables et immuables. Ce code entraîne une erreur :

```rust
    let mut s = String::from("hello");

    let r1 = &s; // pas de problème
    let r2 = &s; // pas de problème
    let r3 = &mut s; // GROS PROBLÈME

    println!("{}, {}, and {}", r1, r2, r3);
```

Voici l'erreur :

```text
    erreur[E0502]: ne peut pas emprunter `s` comme mutable car il est aussi emprunté comme immuable
     --> src/main.rs:6:14
      |
    4 |     let r1 = &s; // pas de problème
      |              -- l'emprunt immuable a lieu ici
    5 |     let r2 = &s; // pas de problème
    6 |     let r3 = &mut s; // GROS PROBLÈME
      |              ^^^^^^ l'emprunt mutable a lieu ici
    7 |
    8 |     println!("{}, {}, and {}", r1, r2, r3);
      |                                -- l'emprunt immuable est utilisé ici plus tard
```

Ouf ! Nous _ne pouvons pas_ non plus avoir une référence mutable pendant que nous avons une immuable. Les utilisateurs d'une référence immuable ne s'attendent pas à ce que les valeurs changent soudainement sous leurs yeux ! Cependant, plusieurs références immuables sont acceptables car personne ne lisant simplement les données n'a la capacité d'affecter la lecture des données par quelqu'un d'autre.