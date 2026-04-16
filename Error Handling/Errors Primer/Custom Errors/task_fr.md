## Erreurs personnalisées

Utiliser des types d'erreurs génériques comme `Box<dyn error::Error>` n'est pas recommandé pour le code de la bibliothèque, où les appelants pourraient vouloir prendre des décisions basées sur le contenu de l'erreur, au lieu de l'imprimer ou de la propager plus loin. Ici, nous définissons un type d'erreur personnalisé pour permettre aux appelants de décider quoi faire ensuite lorsque notre fonction renvoie une erreur.

Faites passer les tests ! Lisez les indices si vous êtes bloqué.

<div class="hint">Cet exercice utilise une version complétée de <code>PositiveNonzeroInteger</code> du
<a href="course://Error Handling/Errors Primer/Positive Nonzero Integer">tâche précédente</a>.</div>

<div class="hint">Vous pouvez créer une autre fonction à l'intérieur de <code>impl ParsePosNonzeroError</code>.

Par exemple, il serait utile de construire `ParsePosNonzeroError` à partir de `ParseIntError` qui peut être
retourné par la fonction `parse`.
</div>

<div class="hint">Sous la ligne qu'un commentaire vous demande de changer, il y a un exemple d'utilisation
de la méthode <code>map_err()</code> sur un <code>Result</code> pour transformer un type d'erreur en
un autre. Essayez d'utiliser quelque chose de similaire sur le <code>Result</code> de <code>parse()</code>. Vous
pourriez utiliser l'opérateur <code>?</code> pour retourner rapidement de la fonction, ou vous pourriez
utiliser une expression <code>match</code>, ou peut-être qu'il y a une autre façon de faire !</div>

<div class="hint">Lisez-en plus sur <code>map_err()</code> dans la <code>std::result</code> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err">documentation</a>.
</div>