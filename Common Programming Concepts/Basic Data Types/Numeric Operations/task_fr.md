## Opérations numériques

Rust prend en charge les opérations mathématiques de base que vous attendez pour tous les types numériques : addition, soustraction, multiplication, division et reste. Le code suivant montre comment vous utiliseriez chacune d'entre elles dans une déclaration `let` :

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // soustraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // reste
    let remainder = 43 % 5;
}
```

Chaque expression dans ces déclarations utilise un opérateur mathématique et évalue à une seule valeur, qui est ensuite liée à une variable. [L'Annexe B](https://doc.rust-lang.org/stable/book/appendix-02-operators.html) contient une liste de tous les opérateurs que fournit Rust.