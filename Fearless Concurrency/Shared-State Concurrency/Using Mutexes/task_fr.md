### Utiliser des mutex pour permettre l'accès aux données à un seul thread à la fois

_Mutex_ est une abréviation pour _exclusion mutuelle_, c'est-à-dire qu'un mutex permet à un seul thread d'accéder à certaines données à un moment donné. Pour accéder aux données dans un mutex, un thread doit d'abord signaler qu'il veut accéder en demandant d'acquérir le _verrou_ du mutex. Le verrou est une structure de données qui fait partie du mutex et qui garde une trace de qui a actuellement un accès exclusif aux données. Par conséquent, le mutex est décrit comme _protégeant_ les données qu'il détient via le système de verrouillage.

Les mutex ont la réputation d'être difficiles à utiliser car il faut se rappeler deux règles :

*   Vous devez tenter d'acquérir le verrou avant d'utiliser les données.
*   Une fois que vous avez fini avec les données protégées par le mutex, vous devez déverrouiller les données pour que d'autres threads puissent acquérir le verrou.

Pour une métaphore concrète d'un mutex, imaginez un débat lors d'une conférence avec un seul microphone. Avant qu'un paneliste puisse parler, il doit demander ou signaler qu'il veut utiliser le microphone. Une fois qu'il a le microphone, il peut parler aussi longtemps qu'il le souhaite, puis passer le microphone au paneliste suivant qui demande à parler. Si un paneliste oublie de transmettre le microphone lorsqu'il a terminé, personne d'autre ne peut parler. Si la gestion du microphone partagé échoue, le débat ne fonctionnera pas comme prévu !

La gestion des mutex peut être extrêmement complexe à bien réaliser, c'est pourquoi tant de gens sont enthousiastes à propos des canaux. Cependant, grâce au système de types et aux règles de propriété de Rust, vous ne pouvez pas vous tromper dans le verrouillage et le déverrouillage.