## Erreurs avancées

Souvenez-vous, dans l'exercice précédent, nous avions plusieurs fonctions de mise en correspondance pour pouvoir traduire les erreurs de bas niveau en notre type d'erreur personnalisé à l'aide de `map_err()` ? Et si nous pouvions plutôt utiliser directement l'opérateur `?` ?

Faites en sorte que ce code compile ! Complétez le code de façon à ce qu'une erreur appropriée soit renvoyée dans chacun des cas dans `main()`. Faites attention aux commentaires et consultez les indices si vous êtes bloqué.

<div class="hint">
Le code d'analyse est maintenant dans une implémentation du trait <code>FromStr</code>. Notez que le code d'analyse utilise <code>?</code> directement, sans aucun appel à <code>map_err()</code>. Il y a une implémentation partielle de l'exemple du trait <code>From</code> que vous devez compléter.
</div>

<div class="hint">
Détails : L'opérateur <code>?</code> appelle <code>From::from()</code> sur le type d'erreur pour le convertir en le type d'erreur du type de retour de la fonction environnante.
</div>
<div class="hint">
Vous devrez écrire une autre implémentation de <code>From</code> qui a un type d'entrée différent.
</div>