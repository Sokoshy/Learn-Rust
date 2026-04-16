## Travailleurs dans le moteur

Faites-moi compiler !

Bien qu'il serait judicieux de lire d'abord les explications.

`Engine` contient un vecteur de `Worker`s. Initialement, ce vecteur est vide, mais nous pouvons facilement ajouter autant de travailleurs que nécessaire. Une fois que le moteur a suffisamment de travailleurs, nous pouvons le faire fonctionner. Le moteur fait tourner ses travailleurs. Les travailleurs font leur travail et écrivent dans le `log`. Qu'est-ce que ce `log` et d'où vient-il ? Eh bien, c'est simplement un vecteur de `String` dans le moteur. Voyez-vous le problème ?

Les travailleurs nécessitent un accès en écriture au log, mais Rust ne nous permettrait jamais d'avoir plusieurs références mutables au log. Ajouter des `mut` ici et là ne fonctionnerait pas, malheureusement. Nous avons besoin de quelque chose appelé [mutabilité intérieure](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value) et de [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) pour donner aux travailleurs un accès en écriture temporaire au log du moteur.

Il y a un autre problème. La structure `Worker` maintient une référence à un log. Nous savons que les références dans les `struct` nécessitent la spécification de durées de vie, mais n'essayez pas de suivre les suggestions du compilateur. Elles ne vous mèneront nulle part. Le véritable problème est que les travailleurs et le moteur ont des durées de vie différentes. Il serait impossible de convaincre le vérificateur d'emprunt que tout va bien en spécifiant des durées de vie. Au lieu de cela, nous devrions utiliser des [pointeurs intelligents Rc&lt;T>](https://doc.rust-lang.org/book/ch15-04-rc.html).

Modifiez les modules `worker` et `engine` de sorte que le programme compile, s'exécute et passe les tests. Ne modifiez pas `main.rs`. Il devrait fonctionner tel quel.

<div class="hint">
Au lieu de suivre les suggestions du compilateur, votre première étape pourrait être de changer le type du log en <code>Rc&lt;RefCell&lt;Vec&lt;String>>></code> (pointeur intelligent sur un <code>RefCell</code> avec un vecteur de chaînes) dans les modules <code>worker</code> et <code>engine</code>. Ensuite, vous devrez corriger le reste du code.
</div>

<div class="hint">
Vous pourriez avoir besoin des méthodes suivantes :

- `Rc::new()`, `Rc::clone()`;
- `RefCell::new()`, `RefCell::borrow()`, et `RefCell::borrow_mut()`.
</div>

<div class="hint">
Faites attention à ne pas détruire le log après l'avoir imprimé. Assurez-vous de vous référer aux entrées du log par des références lorsque vous les imprimez dans <code>Engine::print_log()</code>.
</div>