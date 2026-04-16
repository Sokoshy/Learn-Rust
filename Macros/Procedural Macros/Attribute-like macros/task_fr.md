### Macros de type attribut

Les macros de type attribut sont similaires aux macros de type derive personnalisées, mais au lieu de générer du code pour l'attribut `derive`, elles vous permettent de créer de nouveaux attributs. Elles sont également plus flexibles : `derive` fonctionne uniquement pour les structures (structs) et les énumérations (enums) ; les attributs peuvent être appliqués à d'autres éléments, tels que les fonctions. Voici un exemple d'utilisation d'une macro de type attribut : disons que vous avez un attribut nommé `route` qui annote des fonctions lors de l'utilisation d'un framework d'application web :

```rust
    #[route(GET, "/")]
    fn index() {
```

Cet attribut `#[route]` serait défini par le framework comme une macro procédurale. La signature de la fonction de définition de la macro ressemblerait à ceci :

```rust
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Ici, nous avons deux paramètres de type `TokenStream`. Le premier est pour le contenu de l'attribut : la partie `GET, "/"`. Le second est le corps de l'élément auquel l'attribut est attaché : dans ce cas, `fn index() {}` et le reste du corps de la fonction.

À part cela, les macros de type attribut fonctionnent de la même manière que les macros de type derive personnalisées : vous créez une crate avec le type de crate `proc-macro` et implémentez une fonction qui génère le code souhaité !