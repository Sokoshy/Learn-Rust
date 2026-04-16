## Traitement de la division

C'est un exercice plus important que la plupart des autres ! Vous pouvez le faire !

Voici votre mission, si vous l'acceptez :

1. Complétez la fonction `divide` pour que les quatre premiers tests réussissent.
2. Faites passer les tests restants en complétant les fonctions `result_with_list` et
`list_of_results`.

Consultez le chapitre [Iterateur](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html) du Rust Book et la [documentation sur les itérateurs](https://doc.rust-lang.org/stable/std/iter/).

Faites défiler vers le bas pour un petit indice pour la partie 2, et faites défiler encore plus bas pour un indice majeur. Amusez-vous bien :-)

<div class="hint">
  La fonction <code>divide</code> doit retourner l'erreur correcte lorsque la division égale n'est pas
possible.
</div>

<div class="hint">
La variable <code>division_results</code> doit être collectée dans un type de collection.

La fonction `result_with_list` doit retourner un unique `Result` où le cas de succès est un vecteur d'entiers et le cas d'échec est un `DivisionError`.

La fonction `list_of_results` doit retourner un vecteur de résultats.
</div>