## Plus d'exclamations sans arguments

Refactorons ce code de façon à ce qu'au lieu d'avoir `Hello` et de créer la chaîne dans `fn main`, nous la créons dans `fn with_exclamation` et transférons la chaîne nouvellement créée de `fn with_exclamation` à son appelant.

<div class="hint">
  Arrêtez de lire dès que vous sentez que vous avez assez d'indices :)
  Ou essayez de faire une étape puis corrigez les erreurs du compilateur qui en résultent !

Alors la procédure est la suivante :
- supprimez la première ligne dans `main` qui crée la nouvelle chaîne
- comme `Hello` n'existe plus maintenant, nous ne pouvons pas la passer à `with_exclamation`
- puisque nous ne voulons rien passer à `with_exclamation`, sa signature doit refléter qu'elle ne prend aucun argument
- puisqu'on ne crée plus de nouvelle chaîne dans `main`, nous devons créer une nouvelle chaîne dans `with_exclamation`, de manière similaire à ce que nous faisions dans `main`
</div>