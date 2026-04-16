#### Un raccourci pour propager les erreurs : l'opérateur ?

Le fragment de code ci-dessous montre une implémentation de `read_username_from_file` qui a la même fonctionnalité que dans l'exemple précédent, mais cette implémentation utilise l'opérateur `?`.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
```

##### Une fonction qui retourne les erreurs au code appelant en utilisant l'opérateur ?

Le `?` placé après une valeur de type `Result` est défini pour fonctionner presque de la même manière que les expressions `match` que nous avons définies pour gérer les valeurs `Result` dans l'exemple causant l'erreur sur `match`. Si la valeur de `Result` est un `Ok`, la valeur à l'intérieur de `Ok` sera retournée par cette expression, et le programme continuera. Si la valeur est un `Err`, le `Err` sera retourné de toute la fonction comme si nous avions utilisé le mot-clé `return`, de sorte que la valeur d'erreur est propagée au code appelant.

Il y a une différence entre ce que fait l'expression `match` dans cet exemple et ce que fait l'opérateur `?` : les valeurs d'erreur sur lesquelles l'opérateur `?` est appliqué passent par la fonction `from`, définie dans le trait `From` de la bibliothèque standard, qui est utilisée pour convertir les erreurs d'un type dans un autre. Lorsque l'opérateur `?` appelle la fonction `from`, le type d'erreur reçu est converti en le type d'erreur défini dans le type de retour de la fonction en cours. Cela est utile lorsqu'une fonction retourne un type d'erreur pour représenter toutes les façons dont une fonction peut échouer, même si certaines parties peuvent échouer pour de nombreuses raisons différentes. Tant que chaque type d'erreur implémente la fonction `from` pour définir comment se convertir en le type d'erreur retourné, l'opérateur `?` se charge de la conversion automatiquement.

Dans le contexte du dernier fragment, le `?` à la fin de l'appel `File::open` retournera la valeur à l'intérieur d'un `Ok` à la variable `f`. Si une erreur se produit, l'opérateur `?` retournera prématurément de toute la fonction et donnera toute valeur `Err` au code appelant. La même chose s'applique au `?` à la fin de l'appel `read_to_string`.

L'opérateur `?` élimine beaucoup de code répétitif et simplifie l'implémentation de cette fonction. Nous pourrions même raccourcir encore plus ce code en enchaînant les appels de méthodes immédiatement après le `?`, comme le montre l'exemple suivant.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
```

##### Enchaînement des appels de méthodes après l'opérateur ?

Nous avons déplacé la création de la nouvelle `String` dans `s` au début de la fonction ; cette partie n'a pas changé. Au lieu de créer une variable `f`, nous avons enchaîné l'appel à `read_to_string` directement sur le résultat de `File::open("hello.txt")?`. Nous avons toujours un `?` à la fin de l'appel `read_to_string`, et nous retournons toujours une valeur `Ok` contenant le nom d'utilisateur dans `s` lorsque les deux `File::open` et `read_to_string` réussissent, plutôt que de retourner des erreurs. La fonctionnalité est encore la même que dans les exemples précédents ; c'est juste une manière différente et plus ergonomique de l'écrire.

En parlant de différentes manières d'écrire cette fonction, le fragment de code ci-dessous montre qu'il y a un moyen de la rendre encore plus courte.

```rust
    use std::io;
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
```

##### Utiliser fs::read_to_string au lieu d'ouvrir puis de lire le fichier

Lire un fichier dans une chaîne de caractères est une opération assez courante, donc Rust fournit la fonction pratique `fs::read_to_string` qui ouvre le fichier, crée une nouvelle `String`, lit le contenu du fichier, place le contenu dans cette `String` et la retourne. Bien sûr, utiliser `fs::read_to_string` ne nous donne pas l'occasion d'expliquer toute la gestion des erreurs, c'est pourquoi nous l'avons fait de manière plus longue d'abord.