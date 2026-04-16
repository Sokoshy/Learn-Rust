## Maîtriser l'EDI : introduction au refactoring de variables

En 1993, un développeur a écrit un code pour calculer quelle année c'était dix, cinq et un an auparavant.

En 2021, un autre développeur est tombé sur ce code et a remarqué qu'il ne produisait plus de résultats corrects.

Ce développeur a décidé de [refactorer le code](https://fr.wikipedia.org/wiki/Refactorisation) 
et d'extraire `1993` dans une nouvelle variable appelée `année`. 
De cette façon, les futurs développeurs tombant sur ce code pourraient le corriger en ne modifiant qu'un seul endroit au lieu de trois.

Refactorons également le code !

%IDE_NAME% dispose d'une fonctionnalité pratique appelée **"Introduction au refactoring de variables"**, qui vous permet de le faire rapidement.

Sélectionnez toute occurrence de `1993` puis appuyez soit sur &shortcut:IntroduceVariable; 
soit choisissez *Refactor -> Introduire une variable...* dans le menu contextuel.

*Remarque* : vous pouvez également utiliser &shortcut:IntroduceConstant; ou *Refactor -> Introduire une constante...* dans le menu contextuel pour créer des constantes.