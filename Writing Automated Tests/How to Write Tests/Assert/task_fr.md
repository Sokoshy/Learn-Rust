## Jouer avec `assert!`

Les tests sont importants pour s'assurer que votre code fait ce que vous pensez qu'il doit faire.

Ce test a un problème — faites en sorte que le test se compile ! Faites en sorte que le test réussisse ! Faites en sorte que le test échoue !

<div class="hint">
  Vous n'avez même pas besoin d'écrire du code pour tester — vous pouvez simplement tester des valeurs et exécuter cela, même si vous ne le feriez pas dans la vraie vie.
  <code>assert!</code> est une macro qui nécessite un argument.
  Selon la valeur de l'argument, <code>assert!</code> ne fera rien (dans ce cas, le test réussira) ou <code>assert!</code> annulera (dans ce cas, le test échouera).
  Donc, essayez de donner différentes valeurs à <code>assert!</code> et voyez lesquelles se compilent, lesquelles réussissent et lesquelles échouent.
</div>