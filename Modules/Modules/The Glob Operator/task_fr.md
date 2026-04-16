### L'opérateur glob

Si nous voulons amener _tous_ les éléments publics définis dans un chemin dans notre portée, nous pouvons spécifier ce chemin suivi de `*`, l'opérateur glob :

```rust
    use std::collections::*;
```

Cette déclaration `use` importe tous les éléments publics définis dans `std::collections` dans la portée actuelle. Soyez prudent lorsque vous utilisez l'opérateur glob ! Glob peut rendre plus difficile de savoir quels noms sont dans la portée et où un nom utilisé dans votre programme a été défini.

L'opérateur glob est souvent utilisé lors des tests pour amener tout ce qui est testé dans le module `tests`; nous en parlerons dans la section [« Comment écrire des tests »](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#how-to-write-tests) du Chapitre 11. L'opérateur glob est également parfois utilisé dans le cadre du modèle de prélude : consultez [la documentation de la bibliothèque standard](https://doc.rust-lang.org/std/prelude/index.html#other-preludes) pour plus d'informations sur ce modèle.

_Vous pouvez vous référer au chapitre suivant dans le livre The Rust Programming Language : [Amener des chemins dans la portée avec le mot-clé use](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword)_