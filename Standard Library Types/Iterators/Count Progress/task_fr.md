## Compter les progrès

La progression du langage Rust est modélisée à l'aide d'une table de hachage. Le nom de l'exercice est la clé et le progrès est la valeur. Deux fonctions de comptage ont été créées pour compter le nombre d'exercices avec un progrès donné. Ces fonctions de comptage utilisent des boucles impératives. Recréez cette fonctionnalité de comptage en utilisant des itérateurs.

Faites en sorte que le code compile et que les tests passent.

<div class="hint">Étape 1 :
La documentation pour le trait <code>std::iter::Iterator</code> contient de nombreuses méthodes
qui seraient utiles ici.
</div>

<div class="hint">
Étape 2 :
Retournez 0 à partir de <code>count_stack</code> pour que le code compile afin de tester le comptage.
</div>

<div class="hint">La variable <code>stack</code> dans <code>count_stack</code> est une tranche de tables de hachage. Elle doit être
convertie en un itérateur afin d'utiliser les méthodes d'itérateur.
</div>

<div class="hint">La méthode <code>fold</code> peut être utile dans la fonction <code>count_stack</code>.</div>