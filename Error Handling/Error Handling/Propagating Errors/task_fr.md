### Propager les erreurs

Lorsque vous écrivez une fonction dont l'implémentation appelle quelque chose qui pourrait échouer, plutôt que de gérer l'erreur à l'intérieur de cette fonction, vous pouvez renvoyer l'erreur au code appelant afin qu'il puisse décider quoi faire. Cela est connu sous le nom de _propagation_ de l'erreur et donne plus de contrôle au code appelant, qui pourrait avoir plus d'informations ou de logique dictant comment l'erreur devrait être gérée que ce que vous avez à disposition dans le contexte de votre code.

Par exemple, le morceau de code ci-dessous montre une fonction qui lit un nom d'utilisateur à partir d'un fichier. Si le fichier n'existe pas ou ne peut pas être lu, cette fonction renverra ces erreurs au code qui a appelé cette fonction.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
```

##### Une fonction qui renvoie des erreurs au code appelant en utilisant `match`

Cette fonction peut être écrite de manière beaucoup plus courte, mais nous allons commencer par faire beaucoup de choses manuellement afin d'explorer la gestion des erreurs ; à la fin, nous montrerons la manière plus courte. Examinons d'abord le type de retour de la fonction : `Result<String, io::Error>`. Cela signifie que la fonction renvoie une valeur de type `Result<T, E>` où le paramètre générique `T` a été rempli avec le type concret `String` et le type générique `E` a été rempli avec le type concret `io::Error`. Si cette fonction réussit sans aucun problème, le code qui appelle cette fonction recevra une valeur `Ok` qui contient un `String`—le nom d'utilisateur que cette fonction a lu dans le fichier. Si cette fonction rencontre des problèmes, le code qui appelle cette fonction recevra une valeur `Err` qui contient une instance de `io::Error` fournissant plus d'informations sur les problèmes rencontrés. Nous avons choisi `io::Error` comme type de retour de cette fonction car c'est le type de la valeur d'erreur renvoyée par les deux opérations que nous appelons dans le corps de cette fonction qui pourraient échouer : la fonction `File::open` et la méthode `read_to_string`.

Le corps de la fonction commence par appeler la fonction `File::open`. Ensuite, nous gérons la valeur `Result` renvoyée avec un `match` similaire au `match` dans "Utiliser une expression `match` pour gérer les variantes de `Result` qui pourraient être renvoyées", sauf qu'au lieu d'appeler `panic!` dans le cas `Err`, nous retournons prématurément de cette fonction et passons la valeur d'erreur de `File::open` au code appelant comme valeur d'erreur de cette fonction. Si `File::open` réussit, nous stockons le descripteur de fichier dans la variable `f` et continuons.

Ensuite, nous créons un nouveau `String` dans la variable `s` et appelons la méthode `read_to_string` sur le descripteur de fichier dans `f` pour lire le contenu du fichier dans `s`. La méthode `read_to_string` renvoie également un `Result` car elle peut échouer, même si `File::open` a réussi. Nous avons donc besoin d'un autre `match` pour gérer ce `Result` : si `read_to_string` réussit, alors notre fonction a réussi, et nous renvoyons le nom d'utilisateur du fichier qui se trouve maintenant dans `s` enveloppé dans un `Ok`. Si `read_to_string` échoue, nous renvoyons la valeur d'erreur de la même manière que nous avons renvoyé la valeur d'erreur dans le `match` qui a géré la valeur de retour de `File::open`. Cependant, nous n'avons pas besoin de dire explicitement `return`, car c'est la dernière expression de la fonction.

Le code qui appelle ce code gérera ensuite soit la réception d'une valeur `Ok` contenant un nom d'utilisateur, soit une valeur `Err` contenant un `io::Error`. Nous ne savons pas ce que le code appelant fera avec ces valeurs. Si le code appelant reçoit une valeur `Err`, il pourrait appeler `panic!` et faire planter le programme, utiliser un nom d'utilisateur par défaut, ou chercher le nom d'utilisateur ailleurs que dans un fichier, par exemple. Nous n'avons pas assez d'informations sur ce que le code appelant essaie réellement de faire, donc nous propageons toutes les informations de succès ou d'erreur vers le haut pour qu'il les gère correctement.

Ce modèle de propagation d'erreurs est si courant dans Rust que Rust fournit l'opérateur point d'interrogation `?` pour faciliter cela.