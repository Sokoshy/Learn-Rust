## Le type caractère

Jusqu'à présent, nous avons travaillé uniquement avec des chiffres, mais Rust prend également en charge les lettres. Le type `char` de Rust est le type alphabétique le plus primitif du langage, et le code suivant montre une manière de l'utiliser. (Notez que les littéraux de type `char` sont spécifiés avec des apostrophes simples, contrairement aux littéraux de chaîne de caractères, qui utilisent des guillemets doubles.)

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let coeur = '❤';
}
```

Le type `char` de Rust occupe quatre octets et représente une Valeur Scalaire Unicode, ce qui signifie qu'il peut représenter bien plus que l'ASCII. Les lettres accentuées; les caractères chinois, japonais et coréens; les emojis; et les espaces à largeur nulle sont tous des valeurs `char` valides en Rust. Les Valeurs Scalaires Unicode vont de `U+0000` à `U+D7FF` et de `U+E000` à `U+10FFFF` inclus. Cependant, un « caractère » n'est pas vraiment un concept en Unicode, donc votre intuition humaine de ce qu’est un « caractère » peut ne pas correspondre à ce qu’un `char` est en Rust. Nous aborderons ce sujet en détail dans [« Stockage de texte codé en UTF-8 avec des chaînes de caractères »](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) au Chapitre 8.