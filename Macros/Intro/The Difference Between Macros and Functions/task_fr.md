### La différence entre macros et fonctions

Fondamentalement, les macros sont un moyen d'écrire du code qui écrit d'autre code, ce qui est connu sous le nom de _métaprogrammation_. Dans l'annexe C, nous discutons de l'attribut `derive`, qui génère une implémentation de divers traits pour vous. Nous avons également utilisé les macros `println!` et `vec!` tout au long du livre. Toutes ces macros _s'étendent_ pour produire plus de code que celui que vous avez écrit manuellement.

La métaprogrammation est utile pour réduire la quantité de code que vous devez écrire et maintenir, ce qui est également l'un des rôles des fonctions. Cependant, les macros ont des capacités supplémentaires que les fonctions n'ont pas.

Une signature de fonction doit déclarer le nombre et le type de paramètres que la fonction possède. Les macros, en revanche, peuvent prendre un nombre variable de paramètres : nous pouvons appeler `println!("hello")` avec un argument ou `println!("hello {}", name)` avec deux arguments. De plus, les macros sont développées avant que le compilateur n'interprète la signification du code, donc une macro peut, par exemple, implémenter un trait sur un type donné. Une fonction ne le peut pas, car elle est appelée à l'exécution et un trait doit être implémenté au moment de la compilation.

L'inconvénient à implémenter une macro au lieu d'une fonction est que les définitions de macros sont plus complexes que les définitions de fonctions, car vous écrivez du code Rust qui écrit du code Rust. En raison de cette indirection, les définitions de macros sont généralement plus difficiles à lire, comprendre et maintenir que les définitions de fonctions.

Une autre différence importante entre macros et fonctions est que vous devez définir les macros ou les importer dans le périmètre _avant_ de les appeler dans un fichier, contrairement aux fonctions que vous pouvez définir n'importe où et appeler n'importe où.