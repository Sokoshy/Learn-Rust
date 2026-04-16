## Construire un tableau de scores

Une liste de scores d'un match de football est donnée (un par ligne). Chaque ligne est de la forme :
```
<nom_équipe_1>,<nom_équipe_2>,<buts_équipe_1>,<buts_équipe_2>
```

Exemple : `Angleterre,France,4,2` (l'Angleterre a marqué 4 buts, la France 2).

Vous devez construire un tableau de scores contenant le nom de l'équipe, les buts marqués par l'équipe et les buts encaissés. Une approche pour construire le tableau de scores est d'utiliser une Hashmap. La solution est partiellement écrite pour utiliser une Hashmap, complétez-la pour passer le test.

Faites-moi passer les tests !

<div class="hint">
Utilisez les méthodes <code>entry()</code> et <code>or_insert()</code> de <code>HashMap</code> pour insérer des entrées correspondant à chaque équipe dans le tableau de scores.

En savoir plus dans [The Book](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value).
</div>

<div class="hint">
S'il y a déjà une entrée pour une clé donnée, la valeur retournée par <code>entry()</code> peut être mise à jour en fonction de la valeur existante.

En savoir plus dans [The Book](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value).
</div>