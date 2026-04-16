## Maîtriser l'IDE : gérer les conditions

Pouvez-vous rapidement comprendre ce que le code du fichier `main.rs` imprime ?

%IDE_NAME% possède de nombreuses fonctionnalités utiles qui peuvent vous aider à rendre le code plus lisible.

Par exemple, appliquons la **commande rapide** de la [loi de DeMorgan](https://fr.wikipedia.org/wiki/Lois_de_De_Morgan).

Placez le curseur sur `&&` et utilisez le raccourci &shortcut:ShowIntentionActions; pour voir les actions contextuelles disponibles.

![](image.png)

Maintenant, pouvez-vous rapidement comprendre ce que le code du fichier `main.rs` imprime ?

<div class="hint">
  La condition if résultante devrait être <br>
  <code>(number > 4 && number <= 9) || (number > 0 && number < 10)</code>
</div>

<div class="hint">
  Le code imprime "If Branch"
</div>

*Note 1*: `&&` et `||` sont des [opérateurs logiques à évaluation courte](https://fr.wikipedia.org/wiki/%C3%89valuation_court-circuit).

*Note 2*: Essayez d'utiliser &shortcut:ShowIntentionActions; à divers endroits dans votre code des tâches précédentes. %IDE_NAME% propose souvent des suggestions utiles pour améliorer votre code.