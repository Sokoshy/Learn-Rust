## Implémenter TryFrom

`TryFrom` est une conversion de type simple et sécurisée qui peut échouer de manière contrôlée dans certaines circonstances. Fondamentalement, c'est la même chose que `From`. La principale différence est que cela doit retourner un type `Result` au lieu du type cible lui-même. Vous pouvez en lire plus à ce sujet sur https://doc.rust-lang.org/std/convert/trait.TryFrom.html

Votre tâche ici est de compléter cette implémentation et de retourner un résultat `Ok` de type interne `Color`. Vous devez créer une implémentation pour un tuple de trois entiers, un tableau de trois entiers et une tranche d'entiers.

Notez que l'implémentation pour le tuple et le tableau sera vérifiée au moment de la compilation, mais la mise en œuvre de la tranche doit vérifier la longueur de celle-ci ! Notez également que les valeurs de couleur RGB correctes doivent être des entiers dans la plage 0..=255.

<div class="hint">Suivez les étapes fournies dans la tâche.
Vous pouvez également utiliser cet <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">exemple</a>.</div>

<div class="hint">Indice : Y a-t-il une implémentation de <code>TryFrom</code> dans la bibliothèque standard qui
peut à la fois effectuer la conversion d'entier requise et vérifier la plage de l'entrée ?</div>

<div class="hint">Regardez les cas de test pour voir quelles variantes d'erreur retourner.</div>

<div class="hint">Vous pouvez utiliser les méthodes <code>map_err</code> ou <code>or</code> de <code>Result</code> pour
convertir les erreurs.</div>

<div class="hint">Si vous souhaitez propager les erreurs en utilisant l'opérateur <code>?</code>
dans votre solution, vous pourriez vouloir consulter cet <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">article</a>.</div>

**Défi** : Pouvez-vous rendre les implémentations de `TryFrom` génériques sur plusieurs types d'entiers ?