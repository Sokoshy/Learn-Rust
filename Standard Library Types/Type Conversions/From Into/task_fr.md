## De From À

Le trait From est utilisé pour les conversions de valeur à valeur.
Si From est correctement implémenté pour un type, le trait Into devrait fonctionner à l'inverse.
Vous pouvez en lire plus à ce sujet sur https://doc.rust-lang.org/std/convert/trait.From.html

Nous implémentons le trait Default pour l'utiliser comme solution de repli
lorsque la chaîne de caractères fournie n'est pas convertible en un objet Person. Votre tâche est de compléter cette implémentation
afin que la ligne `let p = Person::from("Mark,20")` puisse se compiler.
Veuillez noter que vous devrez convertir le composant âge en un `usize`
avec quelque chose comme `"4".parse::<usize>()`. Le résultat de ceci doit être
traité de manière appropriée.

Étapes :  
1. Si la longueur de la chaîne de caractères fournie est 0, alors retournez la valeur par défaut de Person  
2. Séparez la chaîne donnée en utilisant les virgules présentes  
3. Extrayez le premier élément de l'opération de séparation et utilisez-le comme nom  
4. Si le nom est vide, alors retournez la valeur par défaut de Person  
5. Extrayez l'autre élément de l'opération de séparation et convertissez-le en un `usize` comme âge. Si une erreur survient lors de la conversion de l'âge, alors retournez la valeur par défaut de Person. Sinon, retournez un objet Person instancié avec les résultats

<div class="hint">Suivez les étapes fournies juste avant l'implémentation de <code>TryFrom</code>.
Vous pouvez également utiliser cet <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">exemple</a>.</div>