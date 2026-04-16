## Allumer la lampe

Dans cette tâche, nous avons besoin d'une seule lampe et de plusieurs interrupteurs externes, qui peuvent opérer sur la lampe comme dans le scénario suivant :

* créer une lampe
* créer un interrupteur pour la lampe
* créer un autre interrupteur pour la même lampe
* manipuler différents interrupteurs et s'assurer que la lampe est allumée puis éteinte à nouveau

Malheureusement, Rust ne nous permet pas d'avoir plusieurs références mutables à la lampe. Si vous essayez de le faire dans `main.rs`, vous le verrez immédiatement.

Votre objectif est de modifier les modules `lamp` et `switchers` afin que `main` soit compilé et exécutable. Une manière de le faire est d'introduire la *mutabilité intérieure* dans `Lamp` via le [type `std::cell::Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html).

Vous ne devez pas modifier le code dans `main.rs`. Il doit fonctionner tel quel.

<div class="hint">
Continuez à supprimer les modificateurs <code>mut</code> le long du chemin. Pour Rust, tout doit être immuable afin qu'il nous permette d'avoir plusieurs références à la même valeur.
</div>

<div class="hint">Consultez la <a href="https://doc.rust-lang.org/std/cell/struct.Cell.html">documentation</a> pour savoir comment accéder et modifier une valeur dans <code>Cell</code>.</div>