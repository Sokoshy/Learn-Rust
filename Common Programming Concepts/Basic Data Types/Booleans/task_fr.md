## Le type booléen

Comme dans la plupart des autres langages de programmation, un type booléen en Rust a deux valeurs possibles : `true` et `false`. Les booléens occupent un octet en mémoire. Le type booléen en Rust est spécifié en utilisant `bool`. Par exemple :

```rust
fn main() {
    let t = true;

    let f: bool = false; // avec annotation de type explicite
}
```

La manière principale d'utiliser les valeurs booléennes est à travers des conditionnels, comme une expression `if`. Nous verrons comment les expressions `if` fonctionnent en Rust dans la leçon ["Conditions"](course://Common Programming Concepts/Conditions/Intro) de cette section.