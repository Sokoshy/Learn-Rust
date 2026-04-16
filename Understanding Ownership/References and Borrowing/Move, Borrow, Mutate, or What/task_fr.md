## Déplacer, emprunter, muter, ou quoi

Faites-moi compiler ! Ne changez rien à part les arguments de la fonction.

<div class="hint">
Notez que <code>fn get_last_char</code> ne doit pas prendre possession 
parce que <code>message</code> est utilisé plus tard dans la fonction <code>main</code>. 
Au lieu de cela, vous pourriez utiliser une référence pour l'emprunter là-bas.
</div>

<div class="hint">
Notez que <code>fn uppercase_and_print</code> est le dernier appel dans <code>main</code>, donc vous pouvez déplacer
<code>message</code> là-bas en toute sécurité. Par conséquent, il n’est pas nécessaire de passer une référence.

Notez également que cette fonction modifie la variable `message`. Vous devez donc la définir comme mutable.
</div>