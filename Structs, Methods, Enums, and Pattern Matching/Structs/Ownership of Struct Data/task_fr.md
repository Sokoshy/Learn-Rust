### Propriété des données de struct

Dans la définition de la struct `User` dans le premier extrait de code de cette section, nous avons utilisé le type `String` possédé plutôt que le type de tranche de chaîne `&str`. C'est un choix délibéré, car nous voulons que les instances de cette struct possèdent toutes leurs données et que ces données soient valides tant que l'ensemble de la struct est valide.

Il est possible pour les structs de stocker des références à des données possédées par autre chose, mais pour cela, il est nécessaire d'utiliser les *durées de vie*, une fonctionnalité de Rust que nous aborderons dans le chapitre "Types génériques, traits et durée de vie". Les durées de vie garantissent que les données référencées par une struct sont valides tant que la struct l'est. Disons que vous essayez de stocker une référence dans une struct sans spécifier de durées de vie, comme ceci, ce qui ne fonctionnera pas :

```rust,ignore,does_not_compile
 struct User {
     username: &str,
     email: &str,
     sign_in_count: u64,
     active: bool,
 }

 fn main() {
     let user1 = User {
         email: "someone@example.com",
         username: "someusername123",
         active: true,
         sign_in_count: 1,
     };
 }
```

Le compilateur se plaindra qu'il nécessite des spécificateurs de durée de vie :

```console
 $ cargo run
    Compilation de structs v0.1.0 (file:///projects/structs)
 error[E0106]: spécificateur de durée de vie manquant
  --> src/main.rs:2:15
   |
 2 |     username: &str,
   |               ^ paramètre de durée de vie nommé attendu
   |
 aide : envisagez d'introduire un paramètre de durée de vie nommé
   |
 1 | struct User<'a> {
 2 |     username: &'a str,
   |

 error[E0106]: spécificateur de durée de vie manquant
  --> src/main.rs:3:12
   |
 3 |     email: &str,
   |            ^ paramètre de durée de vie nommé attendu
   |
 aide : envisagez d'introduire un paramètre de durée de vie nommé
   |
 1 | struct User<'a> {
 2 |     username: &str,
 3 |     email: &'a str,
   |

 error: arrêt en raison de 2 erreurs précédentes

 Pour plus d'informations sur cette erreur, essayez `rustc --explain E0106`.
 error: n'a pas pu compiler `structs`

 Pour en savoir plus, exécutez la commande à nouveau avec --verbose.
```

Dans le chapitre "Types génériques, traits et durée de vie", nous discuterons comment corriger ces erreurs, afin que vous puissiez stocker des références dans des structs, mais pour l'instant, nous corrigerons les erreurs comme celles-ci en utilisant des types possédés comme `String` au lieu de références comme `&str`.