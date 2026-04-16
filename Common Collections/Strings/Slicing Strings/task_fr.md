### Fragmentation des chaînes

Indexation dans une chaîne est souvent une mauvaise idée, car il n'est pas clair quel devrait être le type de retour de l'opération d'indexation de la chaîne : une valeur d'octet, un caractère, un groupe de graphèmes ou une tranche de chaîne. Par conséquent, Rust vous demande d'être plus précis si vous devez vraiment utiliser des indices pour créer des tranches de chaînes. Pour être plus précis dans votre indexation et indiquer que vous souhaitez une tranche de chaîne, plutôt qu'indexer en utilisant `[]` avec un seul nombre, vous pouvez utiliser `[]` avec une plage pour créer une tranche de chaîne contenant des octets particuliers :

```rust
    let hello = "Здравствуйте";

    let s = &hello[0..4];
```

Ici, `s` sera un `&str` qui contient les 4 premiers octets de la chaîne. Plus tôt, nous avons mentionné que chacun de ces caractères faisait 2 octets, ce qui signifie que `s` sera `Зд`.

Que se passerait-il si nous utilisions `&hello[0..1]` ? La réponse : Rust paniquerait à l'exécution de la même manière que si un index invalide était accédé dans un vecteur :

    thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4

Vous devriez utiliser les plages pour créer des tranches de chaînes avec prudence, car cela peut faire planter votre programme.