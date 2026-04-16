### Créer des instances à partir d'autres instances avec la syntaxe de mise à jour des structures

Il est souvent utile de créer une nouvelle instance d'une structure qui utilise la plupart des valeurs d'une ancienne instance, mais en modifie certaines. Vous le ferez en utilisant la *syntaxe de mise à jour des structures*.

Tout d'abord, l'exemple ci-dessous montre comment nous créons une nouvelle instance `User` dans `user2` sans utiliser la syntaxe de mise à jour. Nous définissons de nouvelles valeurs pour `email` et `username`, mais nous utilisons par ailleurs les mêmes valeurs de `user1` que nous avons créées dans le deuxième exemple de code de cette section.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
```

#### Créer une nouvelle instance `User` en utilisant certaines des valeurs de `user1`

En utilisant la syntaxe de mise à jour des structures, nous pouvons obtenir le même effet avec moins de code, comme montré ci-dessous. La syntaxe `..` spécifie que les champs restants qui ne sont pas explicitement définis doivent avoir la même valeur que les champs de l'instance donnée.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```

#### Utiliser la syntaxe de mise à jour des structures pour définir de nouvelles valeurs `email` et `username` pour une instance `User` mais utiliser le reste des valeurs des champs de l'instance dans la variable `user1`

Le code ci-dessus crée également une instance dans `user2` qui a une valeur différente pour `email` et `username` mais a les mêmes valeurs pour les champs `active` et `sign_in_count` de `user1`.