### Valeurs d'enum avec des données différentes

Il y a un autre avantage à utiliser un enum plutôt qu'une struct : chaque variante peut avoir des types et quantités de données associées différents. Les adresses IP de type version quatre auront toujours quatre composants numériques dont les valeurs seront comprises entre 0 et 255. Si nous voulions stocker les adresses `V4` en tant que quatre valeurs `u8` mais exprimer les adresses `V6` comme une seule valeur `String`, nous ne pourrions pas le faire avec une struct. Les enums gèrent ce cas aisément :

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Nous avons montré plusieurs manières différentes de définir des structures de données pour stocker les adresses IP de version quatre et six. Cependant, il s'avère que vouloir stocker des adresses IP et coder leur type est si courant que [la bibliothèque standard a une définition que nous pouvons utiliser !][IpAddr] Regardons comment la bibliothèque standard définit `IpAddr` : elle a exactement l'enum et les variantes que nous avons définies et utilisées, mais elle intègre les données d'adresse à l'intérieur des variantes sous la forme de deux structs différentes, qui sont définies différemment pour chaque variante :

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Ce code illustre que vous pouvez mettre n'importe quel type de données à l'intérieur d'une variante d'enum : chaînes de caractères, types numériques, ou structs, par exemple. Vous pouvez même inclure un autre enum ! De plus, les types de la bibliothèque standard ne sont souvent pas beaucoup plus compliqués que ce que vous pourriez concevoir.

Notez que même si la bibliothèque standard contient une définition pour `IpAddr`, nous pouvons toujours créer et utiliser notre propre définition sans conflit parce que nous n'avons pas inclus la définition de la bibliothèque standard dans notre portée. Nous parlerons plus de l'importation de types dans la portée [plus tard](course://Modules/Modules/Bringing Paths into Scope with the use Keyword).