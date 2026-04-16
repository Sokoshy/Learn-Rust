## Activer un linter externe

Le plugin IntelliJ Rust ne détecte pas toutes les erreurs. Il s'appuie sur le compilateur Rust pour cela. Pendant que vous apprenez Rust, il est utile de voir les erreurs au fur et à mesure que vous tapez. Pour obtenir ce comportement, nous recommandons d'activer un linter externe comme suit :

1. Allez dans **Paramètres / Préférences | Langues & Frameworks | Rust | Linters externes** (ou directement **Paramètres / Préférences | Rust | Linters externes** si ouvert dans RustRover).
2. Réglez les paramètres comme suit : 
    - Sélectionnez **Cargo Check** dans la liste **Outil externe :** ;
    - Cochez la case **Exécuter un linter externe pour analyser le code à la volée**.

![Linters externes](images/external-linters.png)
3. Appuyez sur **OK**.

Une fois que vous faites cela, %IDE_NAME% rapportera toutes les erreurs détectées par le plugin IntelliJ Rust ou le compilateur Rust.