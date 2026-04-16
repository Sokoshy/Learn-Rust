## Retourner un Result

Cette fonction refuse de générer du texte à imprimer sur un badge si vous lui passez une chaîne vide. Il serait plus agréable qu'elle explique quel est le problème, plutôt que de simplement retourner `None` à certains moments.

<div class="hint">
Pourquoi ne pas utiliser le type <code>Result</code> au lieu de <code>Option</code> ?
</div>

<div class="hint">
Pour effectuer ce changement, vous devrez :

- mettre à jour le type de retour dans la signature de la fonction pour qu'il soit un `Result<String, String>` qui pourrait être les variantes `Ok(String)` et `Err(String)`
- modifier le corps de la fonction pour qu'il retourne `Ok(truc)` là où il retourne actuellement `Some(truc)`
- modifier le corps de la fonction pour qu'il retourne `Err(message d'erreur)` là où il retourne actuellement `None`
</div>