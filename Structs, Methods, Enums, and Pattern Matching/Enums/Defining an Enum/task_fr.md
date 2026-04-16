## Définir une énumération

Observons une situation que nous pourrions vouloir exprimer en code et voyons pourquoi les énumérations sont utiles et plus appropriées que les structures dans ce cas. Supposons que nous devions travailler avec des adresses IP. Actuellement, deux normes majeures sont utilisées pour les adresses IP : la version quatre et la version six. Ce sont les seules possibilités pour une adresse IP que notre programme rencontrera : nous pouvons *énumérer* toutes les variantes possibles, d'où le nom d'énumération.

Toute adresse IP peut être soit une adresse de version quatre, soit une adresse de version six, mais pas les deux en même temps. Cette propriété des adresses IP rend la structure de données énumération appropriée parce que les valeurs d'énumération ne peuvent être qu'une de ses variantes. Les adresses de version quatre et six sont toujours fondamentalement des adresses IP, donc elles devraient être traitées comme le même type lorsque le code gère des situations qui s'appliquent à n'importe quel type d'adresse IP.

Nous pouvons exprimer ce concept en code en définissant une énumération `IpAddrKind` et en listant les types possibles qu'une adresse IP peut être, `V4` et `V6`. Ce sont les variantes de l'énumération :

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` est maintenant un type de données personnalisé que nous pouvons utiliser ailleurs dans notre code.

### Valeurs d'énumération

Nous pouvons créer des instances de chacune des deux variantes de `IpAddrKind` comme ceci :

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Notez que les variantes de l'énumération sont sous-nommées sous son identifiant, et nous utilisons un double deux-points pour les séparer. La raison pour laquelle cela est utile est que maintenant les deux valeurs `IpAddrKind::V4` et `IpAddrKind::V6` sont du même type : `IpAddrKind`. Nous pouvons alors, par exemple, définir une fonction qui prend n'importe quel `IpAddrKind` :

```rust
fn route(ip_kind: IpAddrKind) {}
```

Et nous pouvons appeler cette fonction avec n'importe quelle variante :

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```
