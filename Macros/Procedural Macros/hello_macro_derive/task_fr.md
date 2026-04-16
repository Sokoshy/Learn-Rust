### Étape 2 : Définir une macro procédurale

La prochaine étape est de définir la macro procédurale. Au moment de la rédaction de ce document, les macros procédurales doivent se trouver dans leur propre crate. Finalement, cette restriction pourrait être levée. La convention pour structurer les crates et les crates de macro est la suivante : pour une crate nommée `foo`, une crate de macro procédurale de dérivation personnalisée s'appelle `foo_derive`.

Voici notre nouvelle crate `hello_macro_derive` définie dans `Cargo.toml`.

Nos deux crates sont étroitement liées, nous créons donc la crate de macro procédurale dans le répertoire de notre crate `hello_macro`. Si nous changeons la définition du trait dans `hello_macro`, nous devrons également modifier l'implémentation de la macro procédurale dans `hello_macro_derive`. Les deux crates devront être publiées séparément, et les programmeurs utilisant ces crates devront ajouter les deux comme dépendances et les inclure dans leur scope. Nous pourrions plutôt faire en sorte que la crate `hello_macro` utilise `hello_macro_derive` comme dépendance et ré-exporte le code de la macro procédurale. Cependant, la façon dont nous avons structuré le projet permet aux programmeurs d'utiliser `hello_macro` même s'ils ne souhaitent pas la fonctionnalité `derive`.

Nous devons déclarer la crate `hello_macro_derive` comme une crate de macro procédurale. Nous aurons également besoin des fonctionnalités des crates `syn` et `quote`, comme vous le verrez bientôt, nous devons donc les ajouter comme dépendances. Ajoutez ce qui suit au fichier _Cargo.toml_ pour `hello_macro_derive` :

```toml
    [lib]
    proc-macro = true

    [dependencies]
    syn = "1.0"
    quote = "1.0"
```

Pour commencer à définir la macro procédurale, placez le code du fragment ci-dessous dans votre fichier _src/lib.rs_ pour la crate `hello_macro_derive`. Notez que ce code ne se compilera pas tant que nous n'aurons pas ajouté une définition pour la fonction `impl_hello_macro`.

```rust
    extern crate proc_macro;

    use crate::proc_macro::TokenStream;
    use quote::quote;
    use syn;

    #[proc_macro_derive(HelloMacro)]
    pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        // Construire une représentation du code Rust comme un arbre syntaxique
        // que nous pouvons manipuler
        let ast = syn::parse(input).unwrap();

        // Construire l'implémentation du trait
        impl_hello_macro(&ast)
    }
```

##### Code requis par la plupart des crates de macro procédurale pour traiter le code Rust

Notez que nous avons divisé le code en la fonction `hello_macro_derive`, qui est responsable de l'analyse du `TokenStream`, et la fonction `impl_hello_macro`, qui est responsable de la transformation de l'arbre syntaxique : cela rend l'écriture d'une macro procédurale plus pratique. Le code dans la fonction extérieure (`hello_macro_derive` dans ce cas) sera le même pour presque toutes les crates de macro procédurale que vous rencontrez ou créez. Le code que vous spécifiez dans le corps de la fonction interne (`impl_hello_macro` dans ce cas) sera différent selon le but de votre macro procédurale.

Nous avons introduit trois nouvelles crates : `proc_macro`, [`syn`](https://crates.io/crates/syn), et [`quote`](https://crates.io/crates/quote). La crate `proc_macro` est fournie avec Rust, donc nous n'avons pas besoin de l'ajouter aux dépendances dans _Cargo.toml_. La crate `proc_macro` est l'API du compilateur qui nous permet de lire et manipuler le code Rust à partir de notre code.

La crate `syn` analyse le code Rust depuis une chaîne de caractères dans une structure de données sur laquelle nous pouvons effectuer des opérations. La crate `quote` transforme les structures de données `syn` en code Rust. Ces crates simplifient énormément l'analyse de tout type de code Rust que nous pourrions vouloir traiter : écrire un analyseur complet pour le code Rust n'est pas une tâche aisée.

La fonction `hello_macro_derive` sera appelée lorsqu'un utilisateur de notre bibliothèque spécifie `#[derive(HelloMacro)]` sur un type. Cela est possible car nous avons annoté la fonction `hello_macro_derive` ici avec `proc_macro_derive` et spécifié le nom, `HelloMacro`, qui correspond au nom de notre trait ; c'est la convention que suivent la plupart des macros procédurales.

La fonction `hello_macro_derive` convertit d'abord l'`input` d'un `TokenStream` en une structure de données que nous pouvons ensuite interpréter et sur laquelle nous pouvons effectuer des opérations. C'est ici que `syn` entre en jeu. La fonction `parse` dans `syn` prend un `TokenStream` et renvoie une structure `DeriveInput` représentant le code Rust analysé. L'exemple ci-dessous montre les parties pertinentes de la structure `DeriveInput` que nous obtenons de l'analyse de la chaîne de caractères `struct Pancakes;` :

```rust
    DeriveInput {
        // --snip--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
```

##### L'instance DeriveInput que nous obtenons lors de l'analyse du code qui a l'attribut de la macro dans l'extrait "Le code qu'un utilisateur de notre crate pourra écrire en utilisant notre macro procédurale" ci-dessus

Les champs de cette structure montrent que le code Rust que nous avons analysé est une struct unitaire avec l'`ident` (identifiant, signifiant le nom) de `Pancakes`. Il y a d'autres champs dans cette structure pour décrire toutes sortes de code Rust ; consultez [la documentation de `syn` pour `DeriveInput`](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html) pour plus d'informations.

Nous allons bientôt définir la fonction `impl_hello_macro`, où nous allons construire le nouveau code Rust que nous voulons inclure. Mais avant de le faire, notez que la sortie de notre macro de dérivation est aussi un `TokenStream`. Le `TokenStream` retourné est ajouté au code que nos utilisateurs de crate écrivent, de sorte que lorsqu'ils compilent leur crate, ils auront la fonctionnalité supplémentaire que nous fournissons dans le `TokenStream` modifié.

Vous avez peut-être remarqué que nous appelons `unwrap` pour faire en sorte que la fonction `hello_macro_derive` panique si l'appel à la fonction `syn::parse` échoue ici. Il est nécessaire pour notre macro procédurale de paniquer sur les erreurs parce que les fonctions `proc_macro_derive` doivent retourner `TokenStream` plutôt que `Result` pour se conformer à l'API des macros procédurales. Nous avons simplifié cet exemple en utilisant `unwrap`; dans un code de production, vous devriez fournir des messages d'erreur plus spécifiques sur ce qui a mal tourné en utilisant `panic!` ou `expect`.

Maintenant que nous avons le code pour transformer le code Rust annoté d'un `TokenStream` en une instance `DeriveInput`, générons le code qui implémente le trait `HelloMacro` sur le type annoté, comme montré dans le fragment de code ci-dessous.

```rust
    fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}", stringify!(#name));
                }
            }
        };
        gen.into()
    }
```

##### Implémentation du trait HelloMacro à l'aide du code Rust analysé

Nous obtenons une instance de la structure `Ident` contenant le nom (identifiant) du type annoté en utilisant `ast.ident`. La structure dans le fragment avec l'instance DeriveInput montre que lorsque nous exécutons la fonction `impl_hello_macro` sur le code dans l'exemple avec la macro procédurale, l'`ident` que nous obtenons aura le champ `ident` avec une valeur de `"Pancakes"`. Ainsi, la variable `name` dans le fragment ci-dessus contiendra une instance de la structure `Ident` qui, lorsqu'elle est imprimée, sera la chaîne `"Pancakes"`, le nom de la struct dans l'exemple avec la macro procédurale.

La macro `quote!` nous permet de définir le code Rust que nous voulons retourner. Le compilateur attend quelque chose de différent du résultat direct de l'exécution de la macro `quote!`, nous devons donc le convertir en un `TokenStream`. Nous le faisons en appelant la méthode `into`, qui consomme cette représentation intermédiaire et retourne une valeur du type `TokenStream` requis.

La macro `quote!` offre également des mécaniques de templating très intéressantes : nous pouvons entrer `#name`, et `quote!` le remplacera par la valeur dans la variable `name`. Vous pouvez même faire des répétitions similaires à la façon dont fonctionnent les macros normales. Consultez [la documentation de la crate `quote`](https://docs.rs/quote) pour une introduction complète.

Nous voulons que notre macro procédurale génère une implémentation de notre trait `HelloMacro` pour le type annoté par l'utilisateur, que nous pouvons obtenir en utilisant `#name`. L'implémentation du trait a une fonction, `hello_macro`, dont le corps contient la fonctionnalité que nous voulons fournir : imprimer `Hello, Macro! My name is` et ensuite le nom du type annoté.

La macro `stringify!` utilisée ici est intégrée à Rust. Elle prend une expression Rust, telle que `1 + 2`, et à la compilation transforme l'expression en littéral de chaîne, tel que `"1 + 2"`. Cela est différent de `format!` ou `println!`, des macros qui évaluent l'expression et transforment ensuite le résultat en `String`. Il est possible que l'entrée `#name` soit une expression à imprimer littéralement, nous utilisons donc `stringify!`. Utiliser `stringify!` permet également d'économiser une allocation en convertissant `#name` en littéral de chaîne à la compilation.

À ce stade, `cargo build` devrait se terminer avec succès à la fois dans `hello_macro` et `hello_macro_derive`. Connectons ces crates au code dans l'exemple avec la macro procédurale pour voir la macro procédurale en action ! Créez un nouveau projet binaire dans votre répertoire _projects_ en utilisant `cargo new pancakes`. Nous devons ajouter `hello_macro` et `hello_macro_derive` comme dépendances dans le fichier _Cargo.toml_ de la crate `pancakes`. Si vous publiez vos versions de `hello_macro` et `hello_macro_derive` sur _https://crates.io/_, elles seraient des dépendances régulières ; sinon, vous pouvez les spécifier comme dépendances `path` comme suit :

```toml
    [dependencies]
    hello_macro = { path = "../hello_macro" }
    hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

Mettez le code dans l'exemple avec la macro procédurale dans _src/main.rs_, et exécutez `cargo run` : il devrait imprimer `Hello, Macro! My name is Pancakes!`. L'implémentation du trait `HelloMacro` de la macro procédurale a été incluse sans que la crate `pancakes` ait besoin de l'implémenter ; le `#[derive(HelloMacro)]` a ajouté l'implémentation du trait.

Ensuite, explorons comment les autres types de macros procédurales diffèrent des macros de dérivation personnalisées.