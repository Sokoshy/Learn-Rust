### Syntaxe d'annotation de durée de vie

Les annotations de durée de vie ne changent pas la durée de vie des références. Tout comme les fonctions peuvent accepter n'importe quel type lorsque la signature spécifie un paramètre de type générique, les fonctions peuvent accepter des références avec n'importe quelle durée de vie en spécifiant un paramètre de durée de vie générique. Les annotations de durée de vie décrivent les relations des durées de vie de plusieurs références entre elles sans affecter les durées de vie.

Les annotations de durée de vie ont une syntaxe légèrement inhabituelle : les noms des paramètres de durée de vie doivent commencer par une apostrophe (`'`) et sont généralement en minuscules et très courts, comme les types génériques. La plupart des gens utilisent le nom `'a`. Nous plaçons les annotations de paramètres de durée de vie après le `&` d'une référence, en utilisant un espace pour séparer l'annotation du type de la référence.

Voici quelques exemples : une référence à un `i32` sans paramètre de durée de vie, une référence à un `i32` qui a un paramètre de durée de vie nommé `'a`, et une référence mutable à un `i32` qui a également la durée de vie `'a`.

```rust,ignore
&i32        // une référence
&'a i32     // une référence avec une durée de vie explicite
&'a mut i32 // une référence mutable avec une durée de vie explicite
```

Une annotation de durée de vie seule n’a pas beaucoup de sens, car les annotations sont destinées à indiquer à Rust comment les paramètres de durée de vie génériques de plusieurs références se rapportent les uns aux autres. Par exemple, disons que nous avons une fonction avec le paramètre `first` qui est une référence à un `i32` avec la durée de vie `'a`. La fonction a également un autre paramètre nommé `second`, qui est une autre référence à un `i32` ayant aussi la durée de vie `'a`. Les annotations de durée de vie indiquent que les références `first` et `second` doivent toutes deux vivre aussi longtemps que cette durée de vie générique.