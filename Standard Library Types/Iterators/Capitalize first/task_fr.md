## Capitaliser la première lettre

Dans ce module, vous apprendrez certains des avantages uniques que peuvent offrir les itérateurs.

Étape 1. Complétez la fonction `capitalize_first` pour réussir les deux premiers tests. Elle doit retourner "Hello" lorsque l'entrée est "hello" et une chaîne vide lorsque l'entrée est une chaîne vide.

Étape 2. Appliquez la fonction `capitalize_first` à une tranche de sous-chaînes de caractères.
Retournez un vecteur de chaînes.
`["hello", "world"]` -> `["Hello", "World"]`

Étape 3. Appliquez la fonction `capitalize_first` à nouveau à une tranche de sous-chaînes de caractères.
Retournez une seule chaîne.
`["hello", " ", "world"]` -> `"Hello World"`

Comme toujours, il y a des indices !

<div class="hint">
La variable <code>first</code> est un <code>char</code>. Elle doit être mise en majuscule et ajoutée aux
caractères restants dans <code>c</code> pour retourner la <code>String</code> correcte.
Les caractères restants dans <code>c</code> peuvent être vus comme une tranche de chaîne en utilisant la
méthode <code>as_str</code>.
La documentation pour <code>char</code> contient de nombreuses méthodes utiles.
<a href="https://doc.rust-lang.org/std/primitive.char.html">https://doc.rust-lang.org/std/primitive.char.html</a>.</div>

<div class="hint">Créez un itérateur à partir de la tranche. Transformez les valeurs itérées en appliquant
la fonction <code>capitalize_first</code>. N'oubliez pas de collecter l'itérateur.</div>

<div class="hint">C'est étonnamment similaire à la solution précédente. Collect est très puissant
et très général. Rust a juste besoin de connaître le type souhaité.</div>