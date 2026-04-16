## Tâche : Filtrer une séquence avec une fermeture

Vous avez un tableau avec des éléments `i32`, un `cap`, et un compteur `left_behind`. Votre tâche
est de compléter la fermeture donnée en paramètre à la méthode `filter`. La fermeture
doit garder uniquement les éléments strictement inférieurs à `cap`. Elle doit également compter tous les éléments
laissés en arrière.

<div class="hint">
Lisez la documentation sur la méthode <code>filter</code> pour comprendre à quoi
servent ces <code>&&</code> dans l'argument de la fermeture.
</div>

<div class="hint">
Vous voudrez peut-être utiliser le fait que la partie droite de <code> condition || condition </code> est
évaluée seulement si la partie gauche est <code>false</code>.
</div>

<div class="hint">
Utilisez <code>{...; condition}</code> pour faire quelque chose tout en évaluant une certaine expression
logique.
</div>