## Compter les travaux terminés

Faites en sorte que cela compile !

L'idée est que le thread engendré à la ligne 12 termine des travaux pendant que le thread principal surveille la progression jusqu'à ce que 10 travaux soient terminés. En raison de la différence de temps de pause entre les threads engendrés et les threads d'attente, lorsque vous voyez 6 lignes de "attente..." et que le programme se termine sans dépasser le délai du playground, vous l'avez réussi :)

<div class="hint">
  <code>Arc</code> est un pointeur de comptage de références atomique qui permet un accès partagé sûr aux données <b>immutables</b>. Mais nous voulons changer le nombre de <code>jobs_completed</code>, donc nous devrons également utiliser un autre type qui permettra qu'un seul thread modifie les données à la fois.
  Jetez un œil à <a href ="https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct">cette section du livre</a>
  et continuez de faire défiler pour plus d'indices :)
</div>

<div class="hint">
  Avez-vous maintenant un <code>Arc</code> <code>Mutex</code> <code>JobStatus</code> au début de main ? Comme :

  
  <code>let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));</code>
  
  Semblable au code dans l'exemple dans le livre qui intervient après le texte qui dit "Nous pouvons utiliser Arc\<T\> pour résoudre cela.".
  Sinon, essayez cela !
  Si vous le faites et souhaitez plus d'indices, continuez de faire défiler !!
</div>

<div class="hint">
  Assurez-vous qu'aucun de vos threads ne conserve le verrou du mutex pendant qu'ils dorment, car cela empêchera l'autre thread d'obtenir le verrou. Les verrous sont automatiquement libérés lorsqu'ils sortent de la portée.
</div>

<div class="hint">
  D'accord, parlons franchement, c'était en fait difficile pour <i>moi</i> aussi.
  Et je pourrais voir de nombreux problèmes différents que vous pourriez rencontrer, donc à ce stade, je ne suis pas sûr de celui auquel vous êtes confronté :)
  Consultez quelques <a href="https://github.com/carols10cents/rustlings/issues/3 ">réponses</a> possibles -- la mienne est un peu plus compliquée car j'ai décidé que je voulais voir le nombre de travaux actuellement effectués lorsque je vérifiais le statut.

  Veuillez ouvrir une issue si vous rencontrez toujours un problème que ces indices ne vous aident pas à résoudre, ou si vous avez regardé les réponses d'exemple et que vous ne comprenez pas pourquoi elles fonctionnent et que la vôtre ne fonctionne pas.

  Si vous avez appris des solutions d'exemple, je vous encourage à revenir à cet exercice et à l'essayer à nouveau dans quelques jours pour renforcer ce que vous avez appris :)
</div>