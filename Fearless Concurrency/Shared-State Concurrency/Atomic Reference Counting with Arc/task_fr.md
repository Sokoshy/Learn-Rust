#### Comptage de références atomiques avec `Arc<T>`

Heureusement, `Arc<T>` *est* un type semblable à `Rc<T>` qui est sûr à utiliser dans des situations concurrentes. Le *a* signifie *atomique*, ce qui en fait un type *à comptage de références atomique*. Les objets atomiques sont un type supplémentaire de primitives de concurrence que nous ne détaillerons pas ici : consultez la documentation de la bibliothèque standard pour [`std::sync::atomic`] pour plus de détails. À ce stade, vous devez simplement savoir que les objets atomiques fonctionnent comme des types primitifs mais sont sûrs à partager entre plusieurs fils d'exécution.

[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/

Vous pourriez alors vous demander pourquoi tous les types primitifs ne sont pas atomiques et pourquoi les types de la bibliothèque standard ne sont pas implémentés pour utiliser `Arc<T>` par défaut. La raison est que la sécurité des fils d'exécution s'accompagne d'une pénalité de performance que vous ne souhaitez payer que lorsque vous en avez réellement besoin. Si vous effectuez des opérations sur des valeurs au sein d'un seul fil d'exécution, votre code peut s'exécuter plus rapidement s'il n'a pas à appliquer les garanties fournies par les objets atomiques.

Revenons à notre exemple : `Arc<T>` et `Rc<T>` ont la même API, donc nous corrigeons notre programme en modifiant la ligne `use`, l'appel à `new`, et l'appel à `clone`. Le code ci-dessous se compilera enfin et s'exécutera :

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Résultat : {}", *counter.lock().unwrap());
}
```

##### Utilisation d'un Arc<T> pour encapsuler le Mutex<T> afin de pouvoir partager la propriété sur plusieurs fils d'exécution

Ce code affichera le résultat suivant :

```text
    Résultat : 10
```

Nous l'avons fait ! Nous avons compté de 0 à 10, ce qui peut ne pas sembler très impressionnant, mais cela nous a beaucoup appris sur `Mutex<T>` et la sécurité des fils d'exécution. Vous pourriez également utiliser la structure de ce programme pour effectuer des opérations plus complexes que simplement incrémenter un compteur. En utilisant cette stratégie, vous pouvez diviser un calcul en parties indépendantes, répartir ces parties entre les fils d'exécution, puis utiliser un `Mutex<T>` pour que chaque fil mette à jour le résultat final avec sa partie.