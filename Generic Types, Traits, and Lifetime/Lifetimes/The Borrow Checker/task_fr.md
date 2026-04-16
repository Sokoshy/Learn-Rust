### Le vérificateur d'emprunt

Le compilateur Rust possède un *vérificateur d'emprunt* qui compare les portées pour déterminer si tous les emprunts sont valides. L'exemple de code ci-dessous montre le même code que le précédent, mais avec des annotations montrant les durées de vie des variables.

```rust,ignore,does_not_compile
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

#### Annotations des durées de vie de `r` et `x`, nommées respectivement `'a` et `'b`

Ici, nous avons annoté la durée de vie de `r` avec `'a` et la durée de vie de `x` avec `'b`. Comme vous pouvez le voir, le bloc intérieur `'b` est beaucoup plus petit que le bloc de durée de vie extérieur `'a`. Au moment de la compilation, Rust compare la taille des deux durées de vie et voit que `r` a une durée de vie de `'a` mais fait référence à une mémoire ayant une durée de vie de `'b`. Le programme est rejeté parce que `'b` est plus courte que `'a` : l'objet de la référence ne vit pas aussi longtemps que la référence.

L'exemple ci-dessous corrige le code afin qu'il n'ait pas de référence pendante et compile sans erreurs.

```rust
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
```

#### Une référence valide car les données ont une durée de vie plus longue que la référence

Ici, `x` a la durée de vie `'b`, qui dans ce cas est plus grande que `'a`. Cela signifie que `r` peut référencer `x` car Rust sait que la référence dans `r` sera toujours valide tant que `x` est valide.

Maintenant que vous savez où se trouvent les durées de vie des références et comment Rust analyse les durées de vie pour s'assurer que les références seront toujours valides, explorons les durées de vie génériques des paramètres et des valeurs de retour dans le contexte des fonctions.