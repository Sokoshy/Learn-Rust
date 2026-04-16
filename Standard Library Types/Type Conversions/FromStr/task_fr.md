## Implémenter FromStr

Cela fait pratiquement la même chose que `TryFrom<&str>`.
De plus, en implémentant `FromStr`, vous pouvez utiliser la méthode `parse`
sur des chaînes pour générer un objet du type implémenteur.
Vous pouvez en savoir plus à ce sujet sur https://doc.rust-lang.org/std/str/trait.FromStr.html

Étapes :
1. Si la longueur de la chaîne fournie est de 0, alors retournez une erreur.
2. Séparez la chaîne donnée sur les virgules présentes.
3. Seuls 2 éléments doivent être retournés après la séparation, sinon retournez une erreur.
4. Extraire le premier élément de l'opération de séparation et l'utiliser comme nom.
5. Extraire l'autre élément de l'opération de séparation et le convertir en `usize` comme âge
avec quelque chose comme `"4".parse::<usize>()`.
6. Si quelque chose se passe mal lors de l'analyse de l'âge, une erreur doit être retournée.
   Si tout va bien, retournez un `Result` d'un objet `Person`.
   
<div class="hint">L'implémentation de <code>FromStr</code> doit retourner un <code>Ok</code> avec un objet <code>Person</code>,
ou un <code>Err</code> avec une chaîne si la chaîne n'est pas valide. </div>

<div class="hint">C'est presque comme l'exercice from_into, mais en retournant des erreurs au lieu
de revenir à une valeur par défaut.</div>

<div class="hint">Conseil : Regardez les cas de test pour voir quelles variantes d'erreur retourner.</div>

<div class="hint">Un autre conseil : Vous pouvez utiliser la méthode <code>map_err</code> de <code>Result</code> avec une fonction
ou une fermeture pour encapsuler l'erreur de <code>parse::<usize></code>.</div>

<div class="hint">Encore un autre conseil : Si vous souhaitez propager des erreurs en utilisant l'opérateur <code>?</code>
dans votre solution, vous pourriez vouloir consulter <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">cet</a> article.</div>