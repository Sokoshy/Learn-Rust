### Valeurs Enum avec Données

Utiliser des enums présente encore plus d'avantages. En réfléchissant davantage à notre type d'adresse IP, à l'heure actuelle, nous n'avons pas de moyen de stocker les *données* réelles de l'adresse IP ; nous ne savons que de quel *type* elle est. Étant donné que vous venez d'apprendre sur les structures dans le chapitre précédent, vous pourriez aborder ce problème comme montré dans l'exemple de code ci-dessous.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

#### Stocker les données et la variante `IpAddrKind` d'une adresse IP en utilisant une `struct`

Ici, nous avons défini une structure `IpAddr` qui possède deux champs : un champ `kind` qui est de type `IpAddrKind` (l'enum que nous avons défini précédemment) et un champ `address` de type `String`. Nous avons deux instances de cette structure. La première, `home`, a la valeur `IpAddrKind::V4` comme son `kind` avec les données d'adresse associées `127.0.0.1`. La seconde instance, `loopback`, a l'autre variante de `IpAddrKind` comme valeur `kind`, `V6`, et a l'adresse `::1` associée. Nous avons utilisé une structure pour regrouper les valeurs `kind` et `address` ensemble, donc maintenant la variante est associée à la valeur.

Nous pouvons représenter le même concept de façon plus concise en utilisant uniquement un enum, plutôt qu'un enum à l'intérieur d'une structure, en plaçant les données directement dans chaque variante de l'enum. Cette nouvelle définition de l'enum `IpAddr` indique que les variantes `V4` et `V6` auront toutes deux des valeurs `String` associées :

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Nous attachons les données à chaque variante de l'enum directement, il n'y a donc pas besoin d'une structure supplémentaire.