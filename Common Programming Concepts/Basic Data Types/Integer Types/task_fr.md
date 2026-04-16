## Types entiers

Un _entier_ est un nombre sans composante fractionnaire. Nous avons utilisé un type entier dans la Leçon 2, le type `u32`. Cette déclaration de type indique que la valeur associée doit être un entier non signé (les types d'entiers signés commencent par `i` au lieu de `u`) qui occupe 32 bits d'espace. Le tableau ci-dessous montre les types d'entiers intégrés en Rust. Chaque variante dans les colonnes Signé et Non signé (par exemple, i16) peut être utilisée pour déclarer le type d'une valeur entière.

| Longueur | Signé | Non signé |
|----------|-------|-----------|
| 8 bits   | i8    | u8        |
| 16 bits  | i16   | u16       |
| 32 bits  | i32   | u32       |
| 64 bits  | i64   | u64       |
| 128 bits | i128  | u128      |
| arch     | isize | usize     |

##### Tableau : Types d'entiers en Rust

Chaque variante peut être soit signée soit non signée et a une taille explicite. _Signé_ et _non signé_ font référence à la possibilité pour le nombre d'être négatif ou positif—en d'autres termes, si le nombre doit avoir un signe (signé) ou s'il sera toujours positif et peut donc être représenté sans signe (non signé). C’est comme écrire des nombres sur papier : lorsque le signe importe, un nombre est affiché avec un signe plus ou un signe moins ; cependant, lorsqu'il est sûr de supposer que le nombre est positif, il est affiché sans signe. Les nombres signés sont stockés en utilisant la représentation en [complément à deux](https://fr.wikipedia.org/wiki/Compl%C3%A9ment_a_deux).

Chaque variante signée peut stocker des nombres de -($2^{n-1}$) à $2^{n - 1}$-1 inclus, où _n_ est le nombre de bits utilisé par la variante. Donc, un `i8` peut stocker des nombres de -($2^7$) à $2^7$-1, ce qui équivaut à -128 à 127. Les variantes non signées peuvent stocker des nombres de 0 à $2^n$-1, donc un `u8` peut stocker des nombres de 0 à $2^8$-1, ce qui équivaut à 0 à 255.

De plus, les types `isize` et `usize` dépendent du type d'ordinateur sur lequel votre programme s'exécute : 64 bits si vous êtes sur une architecture 64 bits et 32 bits si vous êtes sur une architecture 32 bits.

Vous pouvez écrire des littéraux d'entiers dans l'une des formes montrées dans le tableau ci-dessous. Notez que tous les littéraux numériques, sauf le littéral octet, permettent un suffixe de type, comme `57u8`, et `_` en tant que séparateur visuel, tel que `1_000`.

| Littéraux numériques | Exemple     |
|----------------------|-------------|
| Décimal              | 98_222      |
| Hexadécimal          | 0xff        |
| Octal                | 0o77        |
| Binaire              | 0b1111_0000 |
| Octet (u8 uniquement)| b'A'        |

##### Tableau : Littéraux d'entiers en Rust

Alors, comment savoir quel type d'entier utiliser ? Si vous n'êtes pas sûr, les valeurs par défaut de Rust sont généralement de bons choix et les types d'entiers par défaut sont `i32` : ce type est généralement le plus rapide, même sur des systèmes 64 bits. Le principal cas où vous utiliseriez `isize` ou `usize` est lors de l'indexation de quelque sorte de collection.