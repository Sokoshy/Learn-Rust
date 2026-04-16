### Utilisation de la syntaxe abrégée d'initialisation de champ lorsque les variables et les champs ont le même nom

Parce que les noms des paramètres et les noms des champs de la struct sont exactement identiques dans le code ci-dessus, nous pouvons utiliser la syntaxe abrégée d'initialisation de champ pour réécrire `build_user` afin qu'elle se comporte exactement de la même manière mais sans la répétition de `email` et `username`, comme montré ci-dessous.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### Une fonction `build_user` qui utilise la syntaxe abrégée d'initialisation de champ car les paramètres `email` et `username` ont le même nom que les champs de la struct

Ici, nous créons une nouvelle instance de la struct `User`, qui a un champ nommé `email`. Nous voulons définir la valeur du champ `email` à la valeur du paramètre `email` de la fonction `build_user`. Comme le champ `email` et le paramètre `email` portent le même nom, nous avons seulement besoin d'écrire `email` au lieu de `email: email`.