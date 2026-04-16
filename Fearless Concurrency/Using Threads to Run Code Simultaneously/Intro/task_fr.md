## Utilisation des threads pour exécuter du code simultanément

Dans la plupart des systèmes d'exploitation actuels, le code d'un programme exécuté fonctionne dans un _processus_, et le système d'exploitation gère plusieurs processus à la fois. Dans votre programme, vous pouvez également avoir des parties indépendantes qui s'exécutent simultanément. Les fonctionnalités qui permettent de faire fonctionner ces parties indépendantes sont appelées _threads_.

Diviser le calcul dans votre programme en plusieurs threads peut améliorer les performances, car le programme réalise plusieurs tâches en même temps, mais cela ajoute aussi de la complexité. Puisque les threads peuvent s'exécuter simultanément, il n'y a aucune garantie intrinsèque concernant l'ordre dans lequel les parties de votre code sur différents threads s'exécuteront. Cela peut entraîner des problèmes, tels que :

* Des conditions de compétition, où des threads accèdent aux données ou aux ressources dans un ordre incohérent
* Des blocages, où deux threads attendent que l'autre termine d'utiliser une ressource que l'autre thread possède, empêchant ainsi les deux threads de continuer
* Des bugs qui se produisent uniquement dans certaines situations et qui sont difficiles à reproduire et à corriger de manière fiable

Rust tente d'atténuer les effets négatifs de l'utilisation des threads, mais programmer dans un contexte multithread nécessite toujours une réflexion attentive et requiert une structure de code différente de celle des programmes s'exécutant sur un seul thread.

Les langages de programmation implémentent des threads de plusieurs façons. De nombreux systèmes d'exploitation fournissent une API pour créer de nouveaux threads. Ce modèle, où un langage appelle les API du système d'exploitation pour créer des threads, est parfois appelé modèle _1:1_, signifiant un thread d'exploitation par thread de langage.

De nombreux langages de programmation fournissent leur propre implémentation spéciale des threads. Les threads fournis par le langage de programmation sont appelés _green threads_, et les langages qui utilisent ces green threads les exécutent dans le contexte d'un nombre différent de threads du système d'exploitation. Pour cette raison, le modèle à green threads est appelé modèle _M:N_ : il y a `M` green threads par `N` threads du système d'exploitation, où `M` et `N` ne sont pas nécessairement le même nombre.

Chaque modèle a ses propres avantages et compromis, et le compromis le plus important pour Rust est le support du _runtime_. Le _runtime_ est un terme confus et peut avoir différentes significations selon le contexte.

Dans ce contexte, par _runtime_ nous entendons le code inclus par le langage dans chaque binaire. Ce code peut être grand ou petit selon le langage, mais chaque langage autre qu'assembleur aura une certaine quantité de code runtime. Pour cette raison, quand les gens disent qu'un langage n'a "pas de runtime", ils signifient souvent "petit runtime". Les runtimes plus petits ont moins de fonctionnalités mais ont l'avantage de donner des binaires plus petits, ce qui facilite l'association du langage avec d'autres langages dans plus de contextes. Bien que de nombreux langages soient d'accord pour augmenter la taille du runtime en échange de plus de fonctionnalités, Rust doit avoir presque aucun runtime et ne peut compromettre sa capacité à appeler C pour maintenir ses performances.

Le modèle M:N à green threads nécessite un runtime de langage plus grand pour gérer les threads. Ainsi, la bibliothèque standard de Rust ne fournit qu'une implémentation du threading 1:1. Parce que Rust est un langage de bas niveau, il existe des crates qui implémentent le threading M:N si vous préférez échanger du surcoût pour des aspects tels que plus de contrôle sur quels threads s'exécutent quand et des coûts de commutation de contexte plus faibles, par exemple.

Maintenant que nous avons défini les threads dans Rust, explorons comment utiliser l'API liée aux threads fournie par la bibliothèque standard.