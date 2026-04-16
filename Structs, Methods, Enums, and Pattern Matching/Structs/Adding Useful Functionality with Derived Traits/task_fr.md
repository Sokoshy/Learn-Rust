### Ajouter des fonctionnalités utiles avec des traits dérivés

Ce serait bien de pouvoir imprimer une instance de `Rectangle` pendant que nous déboguons notre programme et de voir les valeurs de tous ses champs. Le code ci-dessous tente d’utiliser la macro `println!` comme nous l’avons fait dans les chapitres précédents. Cependant, cela ne fonctionnera pas.

<span class="filename">Nom du fichier : src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

#### Tentative d'imprimer une instance de `Rectangle`

Lorsque nous compilons ce code, nous obtenons une erreur avec ce message principal :

```text
error[E0277]: `Rectangle` ne met pas en œuvre `std::fmt::Display`
```

La macro `println!`, par défaut, utilise des accolades pour un formatage connu sous le nom de `Display` : une sortie destinée à la consommation directe de l'utilisateur final. Les types primitifs que nous avons vu jusqu'à présent implémentent `Display` par défaut car il n'y a qu'une seule façon dont vous voudriez montrer un `1` ou tout autre type primitif à un utilisateur. Mais avec les structs, la manière dont `println!` devrait formater la sortie est moins claire car il y a plus de possibilités d'affichage : voulez-vous des virgules ou pas ? Voulez-vous imprimer les accolades ? Tous les champs doivent-ils être montrés ? En raison de cette ambiguïté, Rust ne tente pas de deviner ce que nous voulons et les structs n'ont pas une implémentation fournie de `Display`.

Si nous continuons à lire les erreurs, nous trouverons cette note utile :

```text
   = help: le trait `std::fmt::Display` n'est pas implémenté pour `Rectangle`
   = note: dans les chaînes de format, vous pouvez utiliser `{:?}` (ou {:#?} pour un joli format) à la place
```

Essayons-le ! L'appel de la macro `println!` ressemblera maintenant à `println!("rect1 is {:?}", rect1);`. Mettre le spécificateur `:?` à l'intérieur des accolades indique à `println!` que nous voulons utiliser un format de sortie appelé `Debug`. Le trait `Debug` nous permet d’imprimer notre struct de manière utile pour les développeurs afin de voir sa valeur pendant que nous déboguons notre code.

Compilez le code avec ce changement. Zut ! Nous avons encore une erreur :

```text
error[E0277]: `Rectangle` ne met pas en œuvre `Debug`
```

Mais encore une fois, le compilateur nous donne une note utile :

```text
    = help: le trait `Debug` n'est pas implémenté pour `Rectangle`
    = note: ajoutez `#[derive(Debug)]` ou implémentez manuellement `Debug`
```

Rust *inclut* bien des fonctionnalités pour imprimer des informations de débogage, mais nous devons explicitement choisir de rendre cette fonctionnalité disponible pour notre struct. Pour ce faire, nous ajoutons l’annotation `#[derive(Debug)]` juste avant la définition de la struct, comme indiqué ci-dessous.

<span class="filename">Nom du fichier : src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

#### Ajouter l'annotation pour dériver le trait `Debug` et imprimer l'instance de `Rectangle` en utilisant le formatage de débogage

Maintenant, lorsque nous exécutons le programme, nous n'obtenons aucune erreur et nous voyons la sortie suivante :

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle { width: 30, height: 50 }
```

Super ! Ce n'est pas la sortie la plus jolie, mais elle montre les valeurs de tous les champs de cette instance, ce qui serait certainement utile lors du débogage. Lorsque nous avons des structs plus grands, il est utile d’avoir une sortie un peu plus facile à lire ; dans ces cas, nous pouvons utiliser `{:#?}` à la place de `{:?}` dans la chaîne `println!`. Lorsque nous utilisons le style `{:#?}` dans l'exemple, la sortie ressemblera à ceci :

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Rust nous a fourni un certain nombre de traits à utiliser avec l'annotation `derive` qui peuvent ajouter un comportement utile à nos types personnalisés. Ces traits et leurs comportements sont répertoriés dans [l'annexe C][app3] du Livre Rust. Nous couvrirons comment implémenter ces traits avec un comportement personnalisé ainsi que comment créer vos propres traits dans le chapitre "Types génériques, traits et durées de vie".

Notre fonction `area` est très spécifique : elle ne calcule que l'aire des rectangles. Il serait utile de lier ce comportement plus étroitement à notre struct `Rectangle` car il ne fonctionnera avec aucun autre type. Voyons comment nous pouvons continuer à refactorer ce code en transformant la fonction `area` en une *méthode* `area` définie sur notre type `Rectangle`.

[app3]: https://github.com/rust-lang/book/blob/master/src/appendix-03-derivable-traits.md