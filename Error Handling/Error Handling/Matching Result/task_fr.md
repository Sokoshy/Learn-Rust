## Correspondance de Résultat

Disons que nous écrivons un jeu où vous pouvez acheter des articles avec des jetons.  
Tous les articles coûtent 5 jetons, et chaque fois que vous achetez des articles, il y a des frais de traitement de 1 jeton.  
Un joueur du jeu va taper combien d'articles il veut acheter, et la fonction `total_cost` va calculer le nombre total de jetons.  
Cependant, puisque le joueur a tapé la quantité, nous la recevons sous forme de chaîne de caractères -- et il pourrait avoir tapé n'importe quoi, pas seulement des chiffres !

En ce moment, cette fonction ne gère pas du tout le cas d'erreur (et ne gère pas non plus correctement le cas de succès).  
Ce que nous voulons faire est : si nous appelons la fonction `parse` sur une chaîne qui n'est pas un nombre, cette fonction renverra un `ParseIntError`, et dans ce cas, nous voulons immédiatement retourner cette erreur depuis notre fonction et ne pas essayer de multiplier et d'ajouter.

Il y a au moins deux manières d'implémenter cela qui sont toutes deux correctes -- mais l'une est beaucoup plus courte !  
Faites défiler vers le bas pour des indices sur ces deux manières.

<div class="hint">
  Une façon de gérer cela est d'utiliser une instruction <code>match</code> sur <code>item_quantity.parse::&lt;i32&gt()</code> où les cas sont <code>Ok(something)</code> et <code>Err(something)</code>.
  Ce schéma est très courant en Rust, cependant, il existe un opérateur <code>?</code> qui fait à peu près ce que vous feriez avec cette instruction match !  
  Jetez un œil à <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator">cette section du chapitre sur la gestion des erreurs</a> et essayez-le !
</div>