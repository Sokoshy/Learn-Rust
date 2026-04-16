## Envie de glace ?

Nous avons la fonction `maybe_ice_cream` qui retourne combien de glace il reste dans le réfrigérateur.
Si c'est avant 22h, il reste 5 morceaux. À 22h, quelqu'un les mange 
tous, donc il n'en restera plus :(

Votre tâche est de compléter l'implémentation de la fonction `maybe_ice_cream` et de corriger le test qui suit.

Pour en savoir plus sur Option<T>, consultez ces liens :

- [Format Enum Option](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Documentation du module Option](https://doc.rust-lang.org/std/option/)
- [Documentation de l'enum Option](https://doc.rust-lang.org/std/option/enum.Option.html)

<div class="hint">Les options peuvent avoir une valeur <code>Some</code>, avec une valeur interne, ou une valeur <code>None</code>, sans valeur interne.
Il existe plusieurs manières d'obtenir la valeur interne, vous pouvez utiliser <code>unwrap</code>, ou faire un pattern match. Déballer
est le plus simple, mais comment le faire en toute sécurité pour éviter qu'il ne plante plus tard?</div>