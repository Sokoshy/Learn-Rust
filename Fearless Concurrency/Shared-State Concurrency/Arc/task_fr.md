## Utiliser `Arc`

Faites en sorte que ce code compile en remplissant une valeur pour `shared_numbers` et en créant une liaison initiale pour `child_numbers` quelque part. Essayez de ne pas créer de copies du vecteur `numbers` !

Consultez le chapitre [Concurrence à état partagé](https://doc.rust-lang.org/book/2018-edition/ch16-03-shared-state.html) du livre Rust.

<div class="hint">

  Faites en sorte que `shared_numbers` soit un `Arc` du vecteur numbers.
  Ensuite, pour éviter de créer une copie de `numbers`, vous devrez créer `child_numbers` à l'intérieur de la boucle mais toujours dans le thread principal.

  `child_numbers` devrait être un clone de l'Arc des nombres au lieu d'une copie locale au thread des nombres.
</div>

<div class="hint">C'est un exercice simple si vous comprenez les concepts sous-jacents, mais si c'est trop difficile, pensez à lire (ou terminer) la section "Concurrence sans peur" d'abord. Sinon, lisez tout le <a href="https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html">chapitre 16</a> du livre.
</div>