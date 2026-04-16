## Utiliser Clippy comme linter externe

L'outil `Clippy` est un ensemble d'analyses pour étudier votre code afin de détecter les erreurs courantes et améliorer votre code Rust. Pour utiliser `Clippy` comme linter externe, suivez les instructions de [la tâche précédente](course://Introduction/Getting started/External Linter) mais cette fois sélectionnez **Clippy** au lieu de **Cargo Check**.

Consultez les suggestions de clippy et appliquez-les pour résoudre l'exercice. Notez que le bouton **Check** ne vérifie rien vraiment dans cette tâche. Utilisez votre propre jugement ici !

<div class="hint">
Rust stocke la version de la plus haute précision de toute constante mathématique longue ou de précision infinie dans la <a href="https://doc.rust-lang.org/stable/std/f32/consts/index.html">bibliothèque standard de Rust</a>.

Nous pouvons être tentés d'utiliser nos propres approximations pour certaines constantes mathématiques, mais clippy reconnaît ces constantes mathématiques imprécises comme une source potentielle d'erreur. Consultez les suggestions de l'avertissement `clippy` dans la sortie de compilation et utilisez la constante de remplacement appropriée de `std::f32::consts`.
</div>