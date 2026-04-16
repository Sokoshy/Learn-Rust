## Joindre tous les handles

Un défi avec les applications multi-threadées est que le thread principal peut se terminer avant que les threads engendrés ne soient complétés. La fonction `thread::spawn` retourne la structure `JoinHandle`. Rassemblez les JoinHandles et attendez qu'ils se terminent.

<div class="hint">
Placez la valeur que vous avez obtenue de la fonction <code>thread::spawn</code> dans le vecteur <code>handles</code>.
</div>

<div class="hint">
Pour joindre le thread, vous devez appeler <code>join</code> sur le <code>JoinHandle</code> correspondant, puis <code>unwrap</code> le résultat.
</div>