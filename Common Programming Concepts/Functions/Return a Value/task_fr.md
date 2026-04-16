## Retourner une valeur

Faites-moi compiler !

<div class="hint">
  C'est une erreur très courante qui peut être corrigée en supprimant un caractère.

  Cela se produit parce que Rust distingue entre les expressions et les instructions : les expressions renvoient une valeur basée sur leur opérande, et les instructions renvoient simplement un type `()`, qui se comporte exactement comme `void` dans le langage C/C++.

  Nous voulons retourner une valeur de type `i32` depuis la fonction `square`, mais elle retourne un type `()`...

  Ils ne sont pas identiques. Il y a deux solutions :
  1. Ajouter un `return` devant `num * num;`
  2. enlever `;`, pour que ce soit `num * num`
</div>