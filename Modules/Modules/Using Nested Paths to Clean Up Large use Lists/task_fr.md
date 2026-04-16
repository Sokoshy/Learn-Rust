### Utilisation de chemins imbriqués pour nettoyer de grandes listes de use

Si nous utilisons plusieurs éléments définis dans le même crate ou le même module, lister chaque élément sur sa propre ligne peut prendre beaucoup d'espace vertical dans nos fichiers. Par exemple, ces deux déclarations `use` amènent des éléments de `std` dans la portée :

```rust
    use std::cmp::Ordering;
    use std::io;
    // ---snip---
```

Au lieu de cela, nous pouvons utiliser des chemins imbriqués pour amener les mêmes éléments dans la portée sur une seule ligne. Nous le faisons en spécifiant la partie commune du chemin suivie de deux deux-points et d'une liste des parties des chemins qui diffèrent entre accolades, comme montré ci-dessous :

```rust
    use std::{cmp::Ordering, io};
    // ---snip---
```

##### Spécifier un chemin imbriqué pour amener plusieurs éléments avec le même préfixe dans la portée

Dans des programmes plus grands, amener de nombreux éléments dans la portée à partir du même crate ou module en utilisant des chemins imbriqués peut réduire considérablement le nombre de déclarations `use` séparées nécessaires !

Nous pouvons utiliser un chemin imbriqué à n'importe quel niveau dans un chemin, ce qui est utile lorsqu'on combine deux déclarations `use` partageant un sous-chemin. Par exemple, l'extrait ci-dessous montre deux déclarations `use` : l'une qui amène `std::io` dans la portée et l'autre qui amène `std::io::Write` dans la portée.

```rust
    use std::io;
    use std::io::Write;
```

##### Deux déclarations use où l'une est un sous-chemin de l'autre

La partie commune de ces deux chemins est `std::io`, et c'est le chemin complet du premier. Pour fusionner ces deux chemins en une seule déclaration `use`, nous pouvons utiliser `self` dans le chemin imbriqué, comme montré dans l'exemple ci-dessous.

```rust
    use std::io::{self, Write};
```

##### Combiner les chemins en une seule déclaration use

Cette ligne amène `std::io` et `std::io::Write` dans la portée.