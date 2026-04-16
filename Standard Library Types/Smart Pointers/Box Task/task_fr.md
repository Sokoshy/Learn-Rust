## Utiliser un Box

Au moment de la compilation, Rust a besoin de connaître l'espace qu'un type occupe. Cela devient problématique pour les types récursifs, où une valeur peut inclure comme partie d'elle-même une autre valeur du même type. Pour contourner ce problème, nous pouvons utiliser un `Box` - un pointeur intelligent utilisé pour stocker des données sur le tas, ce qui nous permet également d'envelopper un type récursif.

Le type récursif que nous implémentons dans cet exercice est la `liste cons` - une structure de données fréquemment rencontrée dans les langages de programmation fonctionnelle. Chaque élément d'une liste cons contient deux composants : la valeur de l'élément actuel et l'élément suivant. Le dernier élément est une valeur appelée `Nil`.

**Étape 1** : utilisez un `Box` dans la définition de l'enum pour que le code compile

**Étape 2** : créez des listes cons à la fois vides et non vides en remplaçant `unimplemented!()`

Consultez les indices si vous êtes bloqué ! :)

<div class="hint">Étape 1 :
Le message du compilateur devrait aider : puisque nous ne pouvons pas stocker la valeur du type réel lors de la manipulation des types récursifs, nous devons stocker une référence (pointeur) à sa valeur. Nous devrions donc placer notre <code>List</code> à l'intérieur d'un <code>Box</code>. Plus de détails dans le livre ici :
<a href="https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes">https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes</a>.
</div>

<div class="hint">
Étape 2 :
Créer une liste vide devrait être assez simple. Pour une liste non vide, gardez à l'esprit que nous voulons utiliser notre "constructeur de liste" Cons. Bien que la liste actuelle soit composée d'entiers (i32), n'hésitez pas à modifier la définition et à essayer d'autres types !</div>