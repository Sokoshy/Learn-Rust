## Définir et instancier des structures

Les structures sont similaires aux tuples, qui ont été abordés dans le chapitre "Concepts de programmation courants". Comme dans le cas des tuples, les éléments d'une structure peuvent être de types différents. Contrairement aux tuples où chaque valeur est identifiée par sa position, dans une structure vous nommez chaque élément de données pour que la signification des valeurs soit claire. Grâce à ces noms assignés, les structures sont plus flexibles que les tuples : vous n'avez pas à vous fier à l'ordre des données pour spécifier ou accéder aux valeurs d'une instance.

Pour définir une structure, nous entrons le mot-clé `struct` et nommons l'ensemble de la structure. Le nom d'une structure doit décrire la signification des éléments de données regroupés. Ensuite, à l'intérieur des accolades, nous définissons les noms et types des éléments de données, appelés *champs*. Par exemple, le listing ci-dessous montre une structure qui stocke des informations sur un compte utilisateur.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### Définition d'une structure `User`

Pour utiliser une structure après l'avoir définie, nous créons une *instance* de cette structure en spécifiant des valeurs concrètes pour chacun des champs. Nous créons une instance en indiquant le nom de la structure puis ajoutons des accolades contenant des paires `clé: valeur`, où les clés sont les noms des champs et les valeurs sont les données que nous voulons stocker dans ces champs. Nous n'avons pas besoin de spécifier les champs dans le même ordre que celui dans lequel nous les avons déclarés dans la structure. En d'autres termes, la définition de la structure est comme un modèle général pour le type, et les instances remplissent ce modèle avec des données particulières pour créer des valeurs du type. Par exemple, nous pouvons déclarer un utilisateur particulier comme montré ci-dessous.

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

#### Création d'une instance de la structure `User`

Pour obtenir une valeur spécifique d'une structure, nous pouvons utiliser la notation par point. Si nous voulions seulement l'adresse email de cet utilisateur, nous pourrions utiliser `user1.email` partout où nous voulions utiliser cette valeur. Si l'instance est mutable, nous pouvons changer une valeur en utilisant la notation par point et en assignant à un champ particulier. Le code ci-dessous montre comment changer la valeur dans le champ `email` d'une instance mutable `User`.

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

#### Changer la valeur dans le champ `email` d'une instance `User`

Notez que l'instance entière doit être mutable ; Rust ne permet pas de marquer seulement certains champs comme mutables. Comme pour toute expression, nous pouvons construire une nouvelle instance de la structure comme dernière expression dans le corps de la fonction pour retourner implicitement cette nouvelle instance.

Le code ci-dessous montre une fonction `build_user` qui retourne une instance `User` avec l'adresse email et le nom d'utilisateur donnés. Le champ `active` reçoit la valeur `true`, et le champ `sign_in_count` reçoit une valeur de `1`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### Une fonction `build_user` qui prend un email et un nom d'utilisateur et retourne une instance `User`

Il est logique de nommer les paramètres de la fonction avec les mêmes noms que les champs de la structure, mais devoir répéter les noms des champs `email` et `username` et des variables est un peu fastidieux. Si la structure avait plus de champs, répéter chaque nom deviendrait encore plus ennuyeux. Heureusement, il existe une astuce pratique !