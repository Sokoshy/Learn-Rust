## Chemins pour référencer un élément dans l'arborescence des modules

Pour indiquer à Rust où trouver un élément dans une arborescence de modules, nous utilisons un _chemin_ de la même manière qu'un chemin pour naviguer dans un système de fichiers. Si nous voulons appeler une fonction, nous devons connaître son chemin.

Un _chemin_ peut prendre deux formes :

* Un _chemin absolu_ commence à partir de la racine d'une crate en utilisant un nom de crate ou le littéral `crate`.
* Un _chemin relatif_ commence à partir du module actuel et utilise `self`, `super` ou un identifiant dans le module actuel.

Les chemins absolus et relatifs sont suivis par un ou plusieurs identifiants séparés par des doubles deux-points (`::`).

Revenons à l'exemple dans le premier extrait de code de la tâche précédente. Comment appelons-nous la fonction `add_to_waitlist` ? C'est comme demander, quel est le chemin de la fonction `add_to_waitlist` ? Dans l'extrait de code ci-dessous, nous avons simplifié notre code en supprimant certains modules et fonctions. Nous montrerons deux manières d'appeler la fonction `add_to_waitlist` depuis une nouvelle fonction `eat_at_restaurant` définie à la racine de la crate. La fonction `eat_at_restaurant` fait partie de l'API publique de notre crate de bibliothèque, donc nous la marquons avec le mot-clé `pub`. Dans la section "Exposer des chemins avec le mot-clé `pub`", nous détaillerons davantage `pub`. Notez que cet exemple ne compilera pas encore ; nous expliquerons pourquoi un peu plus loin.

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Chemin absolu
        crate::front_of_house::hosting::add_to_waitlist();

        // Chemin relatif
        front_of_house::hosting::add_to_waitlist();
    }
```

##### Appeler la fonction add_to_waitlist en utilisant des chemins absolus et relatifs

La première fois que nous appelons la fonction `add_to_waitlist` dans `eat_at_restaurant`, nous utilisons un chemin absolu. La fonction `add_to_waitlist` est définie dans la même crate que `eat_at_restaurant`, ce qui signifie que nous pouvons utiliser le mot-clé `crate` pour commencer un chemin absolu.

Après `crate`, nous incluons chacun des modules successifs jusqu'à atteindre `add_to_waitlist`. Vous pouvez imaginer un système de fichiers avec la même structure et nous spécifierions le chemin `/front_of_house/hosting/add_to_waitlist` pour exécuter le programme `add_to_waitlist` ; utiliser le nom `crate` pour commencer depuis la racine de la crate est comme utiliser `/` pour commencer depuis la racine du système de fichiers dans votre terminal.

La deuxième fois que nous appelons `add_to_waitlist` dans `eat_at_restaurant`, nous utilisons un chemin relatif. Le chemin commence par `front_of_house`, le nom du module défini au même niveau de l'arborescence que `eat_at_restaurant`. Ici, l'équivalent système de fichiers serait d'utiliser le chemin `front_of_house/hosting/add_to_waitlist`. Commencer par un nom signifie que le chemin est relatif.

Choisir d’utiliser un chemin relatif ou absolu est une décision que vous prendrez en fonction de votre projet. La décision devrait dépendre de votre probabilité de déplacer la définition de l'élément séparément ou avec le code qui utilise l'élément. Par exemple, si nous déplaçons le module `front_of_house` et la fonction `eat_at_restaurant` dans un module nommé `customer_experience`, nous devrons mettre à jour le chemin absolu vers `add_to_waitlist`, mais le chemin relatif resterait valide. Cependant, si nous déplacions la fonction `eat_at_restaurant` séparément dans un module nommé `dining`, le chemin absolu pour l'appel `add_to_waitlist` resterait le même, mais le chemin relatif devrait être mis à jour. Notre préférence est de spécifier des chemins absolus car il est plus probable que les définitions de code et les appels d'éléments soient déplacés indépendamment les uns des autres.

Essayons de compiler l'extrait de code ci-dessus et voyons pourquoi il ne compile pas encore ! L'erreur que nous obtenons est montrée dans l'extrait de code ci-dessous.

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: module `hosting` est privé
 --> src/main.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^

error[E0603]: module `hosting` est privé
  --> src/main.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^
```

##### Erreurs du compilateur lors de la construction du code de l'exemple ci-dessus

Les messages d'erreur indiquent que le module `hosting` est privé. En d'autres termes, nous avons les bons chemins pour le module `hosting` et la fonction `add_to_waitlist`, mais Rust ne nous laisse pas les utiliser car nous n'avons pas accès aux sections privées.

Les modules ne sont pas utiles uniquement pour organiser votre code. Ils définissent également la _frontière de confidentialité_ de Rust : la ligne qui encapsule les détails de mise en œuvre que le code externe n'a pas le droit de connaître, d'appeler ou de sur lesquels se fier. Donc, si vous voulez rendre un élément comme une fonction ou une structure privée, vous le placez dans un module.

La manière dont la confidentialité fonctionne dans Rust est que tous les éléments (fonctions, méthodes, structures, enums, modules et constantes) sont privés par défaut. Les éléments dans un module parent ne peuvent pas utiliser les éléments privés à l'intérieur des modules enfants, mais les éléments dans les modules enfants peuvent utiliser les éléments dans leurs modules ancêtres. La raison est que les modules enfants enveloppent et cachent leurs détails d'implémentation, mais les modules enfants peuvent voir le contexte dans lequel ils sont définis. Pour continuer avec la métaphore du restaurant, pensez aux règles de confidentialité comme étant similaires au bureau arrière d'un restaurant : ce qui s'y passe est privé pour les clients du restaurant, mais les gestionnaires de bureau peuvent voir et faire tout ce qui se passe dans le restaurant où ils opèrent.

Rust a choisi de faire fonctionner le système de modules de cette manière afin que la dissimulation des détails d'implémentation internes soit par défaut. Ainsi, vous savez quelles parties du code interne vous pouvez changer sans casser le code externe. Mais vous pouvez exposer les parties internes du code des modules enfants aux modules ancêtres externes en utilisant le mot-clé `pub` pour rendre un élément public.