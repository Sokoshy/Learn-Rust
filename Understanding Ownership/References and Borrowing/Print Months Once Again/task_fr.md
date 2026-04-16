## Afficher les mois à nouveau

Il se passe quelque chose d'étrange ici. Le programme se compile sans aucun problème. Nous avons `fn print_months` qui emprunte un tableau et imprime son contenu. Tout va bien. Qu'en est-il de `fn print_months_reversed` ? Ne prend-il pas possession du tableau `months` ? Pourquoi pouvons-nous utiliser le même `months` de nouveau après cela alors ? Eh bien non, la propriété n'est pas transférée ici. La valeur d'un tableau est plutôt copiée. Si vous exécutez le programme, vous le verrez immédiatement.

Refactorisez ce programme pour éviter la copie du tableau en passant à une référence mutable dans `fn print_months_reversed` et corrigez les erreurs du compilateur.

<div class="hint">
La première étape consisterait à changer le type d'argument dans <code>fn print_months_reversed</code>.
</div>

<div class="hint">
Comment devrions-nous appeler la fonction qui emprunte son argument de manière mutable ?
</div>