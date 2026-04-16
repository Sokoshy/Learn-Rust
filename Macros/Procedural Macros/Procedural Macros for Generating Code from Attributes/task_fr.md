### Macros procéduraux pour générer du code à partir d'attributs

La seconde forme de macros est constituée des _macros procédurales_, qui se comportent davantage comme des fonctions (et sont un type de procédure). Les macros procédurales acceptent du code en entrée, opèrent sur ce code et produisent du code en sortie, plutôt que de s'appuyer sur des motifs et de remplacer le code par un autre code, comme le font les macros déclaratives.

Les trois types de macros procédurales (derive personnalisé, de type attribut et de type fonction) fonctionnent de manière similaire.

Lors de la création de macros procédurales, les définitions doivent résider dans leur propre crate avec un type de crate spécial. Cela est dû à des raisons techniques complexes que nous espérons éliminer à l'avenir. L'utilisation de macros procédurales ressemble à l'exemple de code ci-dessous, où `some_attribute` est un espace réservé pour l'utilisation d'une macro spécifique.

```rust
    use proc_macro;

    #[some_attribute]
    pub fn some_name(input: TokenStream) -> TokenStream {
    }
```

##### Un exemple d'utilisation d'une macro procédurale

La fonction qui définit une macro procédurale prend un `TokenStream` en entrée et produit un `TokenStream` en sortie. Le type `TokenStream` est défini par le crate `proc_macro` qui est inclus avec Rust et représente une séquence de tokens. C'est le cœur de la macro : le code source sur lequel la macro opère constitue le `TokenStream` d'entrée, et le code que la macro produit est le `TokenStream` de sortie. La fonction a également un attribut attaché qui spécifie quel type de macro procédurale nous créons. Nous pouvons avoir plusieurs types de macros procédurales dans le même crate.

Jetons un coup d'œil aux différents types de macros procédurales. Nous commencerons par une macro derive personnalisée et expliquerons ensuite les petites différences qui distinguent les autres formes.