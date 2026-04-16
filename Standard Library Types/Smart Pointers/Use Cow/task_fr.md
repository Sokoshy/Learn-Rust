## Utiliser `Cow`

Cet exercice explore le type `Cow`, ou Clone-On-Write.
`Cow` est un pointeur intelligent de type clone-à-l'écriture.
Il peut encapsuler et fournir un accès immuable aux données empruntées, et cloner les données paresseusement lorsqu'une mutation ou une appropriation est nécessaire.
Le type est conçu pour fonctionner avec des données empruntées générales via le trait `Borrow`.

Regardez d'abord la fonction `abs_all` : elle modifie la tranche donnée si elle contient des éléments négatifs.

Votre objectif dans chacun des quatre cas (`case1`, `case2`, `case3`, et `case4`) est de décider si les résultats
des appels aux fonctions `Cow::from` et `abs_all` sont soit `Cow::Borrowed` soit `Cow::Owned`.

<div class="hint">
Assurez-vous de comprendre ce qui est exactement donné à <code>Cow::from</code> dans chaque cas. 
La propriété est-elle transférée dans chacun d'eux ?
</div>

<div class="hint">
Qu'en est-il d'un vecteur dans <code>case3</code> et <code>case4</code>, est-il emprunté ou possédé après le <code>Cow::from</code> ?
</div>

<div class="hint">
Puisque le vecteur dans <code>case4</code> est déjà possédé, le type <code>Cow</code> n'a pas besoin de le cloner
même si nous devons le modifier.
</div>

<div class="hint">Consultez la <a href="https://doc.rust-lang.org/std/borrow/enum.Cow.html">documentation sur le type <code>Cow</code></a>.</div>