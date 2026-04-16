## Erreurs multiples

Ce programme utilise une version complétée du code de la tâche précédente. Il ne se compile pas pour le moment ! Pourquoi ? Utilisez les indices si vous êtes bloqué.

<div class="hint">
Il y a deux types de <code>Result</code> différents produits dans <code>main()</code>, qui sont propagés à l'aide des opérateurs <code>?</code>. Comment déclarer un type de retour depuis <code>main()</code> qui permette les deux ?</div>

<div class="hint">Un autre indice : sous le capot, l'opérateur <code>?</code> appelle <code>From::from</code> sur la valeur d'erreur pour la convertir en un objet trait boxé, un <code>Box&lt;dyn error::Error></code>, qui est polymorphe &mdash; cela signifie qu'une grande variété de types d'erreurs peuvent être retournés par la même fonction car toutes les erreurs agissent de la même manière puisqu'elles implémentent toutes le trait <code>error::Error</code>. Consultez la section "Un raccourci pour propager les erreurs : l'opérateur ?" dans cette <a href="course://Error Handling/Error Handling/Propagating Errors Limitations">tâche</a>.
</div>

<div class="hint">Cet exercice utilise des concepts que nous n'aborderons que plus tard dans le cours, comme <code>Box</code> et le trait <code>From</code>. Il n'est pas important de les comprendre en détail pour le moment, mais vous pouvez lire à l'avance si vous le souhaitez. Lisez plus sur le boxing des erreurs :
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">ici</a>.</div>