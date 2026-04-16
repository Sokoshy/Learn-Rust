## Types à virgule flottante

Rust possède également deux types primitifs pour les `nombres à virgule flottante`, qui sont des nombres avec des points décimaux. Les types à virgule flottante de Rust sont `f32` et `f64`, qui sont respectivement de 32 bits et 64 bits. Le type par défaut est `f64` car sur les processeurs modernes, il a à peu près la même vitesse que `f32` mais est capable de plus de précision.

Voici un exemple qui montre les nombres à virgule flottante en action :

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let z = 4.0f32; // f32
}
```

Les nombres à virgule flottante sont représentés selon la norme IEEE-754. Le type `f32` est un flottant en précision simple, et `f64` a une précision double.