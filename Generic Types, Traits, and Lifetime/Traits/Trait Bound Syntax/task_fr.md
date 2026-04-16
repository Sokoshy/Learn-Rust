## Syntaxe des contraintes de traits

Une école magique imaginaire dispose d'un nouveau système de génération de bulletins créé en Rust ! Actuellement, le système ne prend en charge que la création de bulletins où les notes des étudiants sont représentées numériquement (par exemple, 1.0 -> 5.5). Cependant, l'école délivre également des notes alphabétiques (A+ -> F-) et doit pouvoir imprimer les deux types de bulletins !

Apportez les modifications nécessaires au code dans la structure `ReportCard` et le bloc `impl` pour prendre en charge les bulletins alphabétiques.

<div class="hint">Pour trouver la meilleure solution à ce défi, vous devrez reconsidérer vos connaissances sur les traits, en particulier la syntaxe des contraintes de traits. Vous aurez peut-être aussi besoin de ceci : <code>use std::fmt::Display;</code></div>

<div class="hint">C'est certainement plus difficile que les deux derniers exercices ! Vous devez penser non seulement à rendre la structure <code>ReportCard</code> générique, mais aussi à la propriété correcte - vous devrez également modifier légèrement l'implémentation de la structure... vous pouvez le faire !</div>