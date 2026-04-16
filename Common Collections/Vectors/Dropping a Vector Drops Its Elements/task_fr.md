### La libération d'un vecteur libère ses éléments

Comme toute autre `struct`, un vecteur est libéré lorsqu'il sort de la portée, comme annoté ci-dessous.

```rust
    {
        let v = vec![1, 2, 3, 4];

        // faire des manipulations avec v
    } // <- v sort de la portée et est libéré ici
```

#### Montrer où le vecteur et ses éléments sont libérés

Lorsque le vecteur est libéré, tout son contenu l'est aussi, ce qui signifie que les entiers qu'il contient seront nettoyés. Cela peut sembler évident, mais peut devenir un peu plus compliqué lorsque vous commencez à introduire des références aux éléments du vecteur. Abordons cela ensuite !