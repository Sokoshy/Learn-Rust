## Utiliser `Rc`

Dans cet exercice, nous voulons exprimer le concept de propriétaires multiples via le type `Rc<T>`. Il s'agit d'un modèle de notre système solaire - il y a un type `Sun` et plusieurs `Planet`s. Les `Planet`s prennent possession du soleil, indiquant qu'ils tournent autour du soleil.

Faites en sorte que ce code se compile en utilisant les primitives `Rc` appropriées pour exprimer que le soleil a plusieurs propriétaires.

<div class="hint">
C'est un exercice simple pour utiliser le type <code>Rc<T></code>. Chaque <code>Planet</code> a la possession du <code>Sun</code> et utilise <code>Rc::clone()</code> pour incrémenter le comptage de références du <code>Sun</code>. Après avoir utilisé <code>drop()</code> pour déplacer les Planets hors de la portée individuellement, le comptage de références diminue. À la fin, le soleil n'a à nouveau qu'une seule référence, la sienne. <a href="https://doc.rust-lang.org/book/ch15-04-rc.html">Voir plus dans The Book</a>.
</div>