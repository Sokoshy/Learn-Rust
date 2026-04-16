## Comprendre la propriété

La propriété est la caractéristique la plus unique de Rust, et elle permet à Rust de garantir la sécurité de la mémoire sans avoir besoin d’un ramasse-miettes. Par conséquent, il est important de comprendre comment la propriété fonctionne dans Rust. Dans ce chapitre, nous parlerons de la propriété ainsi que de plusieurs fonctionnalités associées : l'emprunt, les slices et comment Rust organise les données en mémoire.

## Qu'est-ce que la propriété ?

La caractéristique centrale de Rust est _la propriété_. Bien que cette fonctionnalité soit simple à expliquer, elle a de profondes implications pour le reste du langage.

Tous les programmes doivent gérer la façon dont ils utilisent la mémoire de l’ordinateur pendant leur exécution. Certains langages utilisent un ramasse-miettes qui cherche constamment la mémoire qui n’est plus utilisée pendant que le programme s’exécute ; dans d’autres langages, le programmeur doit allouer et libérer explicitement la mémoire. Rust utilise une troisième approche : la mémoire est gérée à travers un système de propriété avec un ensemble de règles que le compilateur vérifie à la compilation. Aucune des fonctionnalités de propriété ne ralentit votre programme pendant son exécution.

Parce que la propriété est un concept nouveau pour de nombreux programmeurs, il faut un certain temps pour s'y habituer. La bonne nouvelle est que plus vous acquérez de l’expérience avec Rust et les règles du système de propriété, plus vous serez en mesure de développer naturellement du code sûr et efficace. Persévérez !

Quand vous comprenez la propriété, vous avez une base solide pour comprendre les fonctionnalités qui rendent Rust unique. Dans ce chapitre, vous apprendrez la propriété en travaillant avec certains exemples qui se concentrent sur une structure de données très courante : les chaînes de caractères.

### La pile et le tas

Dans de nombreux langages de programmation, vous n’avez pas à penser souvent à la pile et au tas. Mais dans un langage de programmation système comme Rust, le fait qu’une valeur soit sur la pile ou sur le tas a un impact plus important sur le comportement du langage et sur les décisions que vous devez prendre. Des parties de la propriété seront décrites en relation avec la pile et le tas plus loin dans ce chapitre, alors voici une brève explication pour vous préparer.

La pile et le tas sont des parties de la mémoire disponibles pour votre code à utiliser au moment de l’exécution, mais elles sont structurées de manière différente. La pile stocke les valeurs dans l'ordre où elle les reçoit et retire les valeurs dans l’ordre inverse. Cela est appelé _dernier entré, premier sorti_. Pensez à une pile d'assiettes : lorsque vous ajoutez plus d'assiettes, vous les mettez au sommet de la pile, et lorsque vous avez besoin d'une assiette, vous en prenez une du sommet. Ajouter ou retirer des assiettes du milieu ou du bas ne fonctionnerait pas aussi bien ! Ajouter des données est appelé _empilement sur la pile_, et retirer des données est appelé _dépilement de la pile_.

Toutes les données stockées sur la pile doivent avoir une taille connue et fixe. Les données dont la taille est inconnue à la compilation ou qui peuvent changer de taille doivent être stockées sur le tas à la place. Le tas est moins organisé : quand vous placez des données sur le tas, vous demandez une certaine quantité d'espace. Le gestionnaire de mémoire trouve un espace vide dans le tas suffisamment grand, le marque comme étant utilisé et retourne un _pointeur_, qui est l'adresse de cet emplacement. Ce processus est appelé _allocation sur le tas_ et est parfois abrégé en simplement _allocation_. Empiler des valeurs sur la pile n'est pas considéré comme une allocation. Comme le pointeur a une taille connue et fixe, vous pouvez stocker le pointeur sur la pile, mais lorsque vous souhaitez avoir les données réelles, vous devez suivre le pointeur.

Pensez à être assis dans un restaurant. Quand vous entrez, vous indiquez le nombre de personnes dans votre groupe, et le personnel trouve une table vide qui convient à tout le monde et vous y conduit. Si quelqu’un de votre groupe arrive en retard, il peut demander où vous êtes assis pour vous retrouver.

Empiler sur la pile est plus rapide que d'allouer sur le tas parce que le gestionnaire de mémoire n'a jamais à chercher un endroit pour stocker les nouvelles données ; cet endroit est toujours au sommet de la pile. En comparaison, allouer de l’espace sur le tas nécessite plus de travail, car le gestionnaire de mémoire doit d'abord trouver un espace suffisamment grand pour contenir les données, puis effectuer la comptabilité pour préparer la prochaine allocation.

Accéder aux données dans le tas est plus lent que d'accéder aux données sur la pile parce que vous devez suivre un pointeur pour y accéder. Les processeurs contemporains sont plus rapides s'ils sautent moins d'un emplacement mémoire à un autre. Poursuivant l'analogie, pensez à un serveur dans un restaurant prenant les commandes de plusieurs tables. Il est plus efficace de prendre toutes les commandes d'une table avant de passer à la suivante. Prendre une commande de la table A, puis une commande de la table B, puis encore une de la table A, et ensuite de la table B serait un processus beaucoup plus lent. De même, un processeur peut faire son travail plus efficacement s'il travaille sur des données proches d'autres données (comme dans la pile) plutôt que plus éloignées (comme cela peut se produire dans le tas). Allouer une grande quantité d'espace sur le tas peut également prendre du temps.

Quand votre code appelle une fonction, les valeurs passées à la fonction (y compris, potentiellement, les pointeurs vers des données dans le tas) et les variables locales de la fonction sont empilées sur la pile. Une fois la fonction terminée, ces valeurs sont dépilées de la pile.

Garder une trace de quelles parties du code utilisent quelles données sur le tas, minimiser la quantité de données dupliquées sur le tas, et nettoyer les données inutilisées sur le tas pour ne pas manquer d’espace sont tous des problèmes que la propriété résout. Une fois que vous comprenez la propriété, vous n’aurez pas besoin de penser souvent à la pile et au tas, mais savoir que la gestion des données sur le tas est la raison d'être de la propriété peut vous aider à expliquer pourquoi elle fonctionne comme elle le fait.

### Règles de la propriété

Tout d'abord, examinons les règles de la propriété. Gardez ces règles en mémoire pendant que nous travaillons sur les exemples qui les illustrent :

* Chaque valeur dans Rust a une variable appelée son _propriétaire_.
* Il ne peut y avoir qu'un seul propriétaire à la fois.
* Quand le propriétaire sort de son périmètre, la valeur sera détruite.