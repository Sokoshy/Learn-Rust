### Annotations de durée de vie dans les définitions de méthode

Lorsque nous implémentons des méthodes sur une struct avec des durées de vie, nous utilisons la même syntaxe que celle des paramètres de type générique présentée dans la section "Un méthode qui utilise des types génériques différents de la définition de sa struct" de la section "Types de données génériques". L'endroit où nous déclarons et utilisons les paramètres de durée de vie dépend de leur relation avec les champs de la struct ou les paramètres et valeurs de retour de la méthode.

Les noms de durée de vie pour les champs de struct doivent toujours être déclarés après le mot-clé `impl` et ensuite utilisés après le nom de la struct, car ces durées de vie font partie du type de la struct.

Dans les signatures de méthode à l’intérieur du bloc `impl`, les références peuvent être liées à la durée de vie des références dans les champs de la struct, ou elles peuvent être indépendantes. De plus, les règles d'omission de durée de vie font souvent que les annotations de durée de vie ne sont pas nécessaires dans les signatures de méthode. Observons quelques exemples en utilisant la struct nommée `ImportantExcerpt` que nous avons définie plus tôt dans cette section.

Tout d'abord, nous utiliserons une méthode nommée `level` dont le seul paramètre est une référence à `self` et dont la valeur de retour est un `i32`, qui n'est pas une référence à quoi que ce soit :

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

La déclaration du paramètre de durée de vie après `impl` et son utilisation après le nom du type sont nécessaires, mais nous ne sommes pas tenus d’annoter la durée de vie de la référence à `self` en raison de la première règle d'omission.

Voici un exemple où la troisième règle d'omission de durée de vie s'applique :

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention s'il vous plaît : {}", announcement);
        self.part
    }
}
```

Il y a deux durées de vie en entrée, donc Rust applique la première règle d'omission de durée de vie et attribue à la fois à `&self` et à `announcement` leurs propres durées de vie. Puis, parce qu'un des paramètres est `&self`, le type de retour obtient la durée de vie de `&self`, et toutes les durées de vie ont été prises en compte.