## L'opérateur de contrôle de flux `match`

Rust possède un opérateur de contrôle de flux extrêmement puissant appelé `match` qui vous permet de comparer une valeur à une série de motifs et ensuite d'exécuter du code en fonction du motif correspondant. Les motifs peuvent être constitués de valeurs littérales, de noms de variables, de jokers et de bien d'autres choses ; le Chapitre 18 du [Livre Rust][book] couvre tous les différents types de motifs et leur fonctionnement. La puissance de `match` provient de l'expressivité des motifs et du fait que le compilateur confirme que tous les cas possibles sont traités.

Pensez à une expression `match` comme à une machine à trier les pièces : les pièces glissent sur une piste avec des trous de différentes tailles, et chaque pièce tombe à travers le premier trou qu'elle rencontre qui lui convient. De la même manière, les valeurs traversent chaque motif dans un `match`, et dès qu'un motif correspond à la valeur, celle-ci tombe dans le bloc de code associé pour être utilisée lors de l'exécution.

Puisque nous venons de mentionner des pièces, utilisons-les comme exemple avec `match` ! Nous pouvons écrire une fonction qui peut prendre une pièce de monnaie inconnue des États-Unis et, semblable à la machine de tri, déterminer de quelle pièce il s'agit et retourner sa valeur en cents, comme montré ci-dessous.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Un enum et une expression `match` ayant les variantes de l'enum comme motifs

Décomposons le `match` dans la fonction `value_in_cents`. Tout d'abord, nous listons le mot-clé `match` suivi d'une expression, qui dans ce cas est la valeur `coin`. Cela semble très similaire à une expression utilisée avec `if`, mais il y a une grande différence : avec `if`, l'expression doit retourner une valeur booléenne, mais ici, elle peut être de n'importe quel type. Le type de `coin` dans cet exemple est l'enum `Coin` que nous avons défini à la ligne 1.

Ensuite viennent les branches de `match`. Une branche comporte deux parties : un motif et du code. La première branche ici a un motif qui est la valeur `Coin::Penny`, puis l'opérateur `=>` qui sépare le motif et le code à exécuter. Le code dans ce cas est simplement la valeur `1`. Chaque branche est séparée de la suivante par une virgule.

Lorsque l'expression `match` s'exécute, elle compare la valeur résultante au motif de chaque branche, dans l'ordre. Si un motif correspond à la valeur, le code associé à ce motif est exécuté. Si ce motif ne correspond pas à la valeur, l'exécution continue vers la branche suivante, un peu comme dans une machine à trier les pièces. Nous pouvons avoir autant de branches que nécessaire : dans l'exemple de code ci-dessus, notre `match` a quatre branches.

Le code associé à chaque branche est une expression, et la valeur résultante de l'expression dans la branche correspondante est la valeur qui est retournée pour l'expression `match` entière.

Les accolades ne sont généralement pas utilisées si le code de la branche match est court, comme dans l'exemple de code ci-dessus où chaque branche retourne simplement une valeur. Si vous souhaitez exécuter plusieurs lignes de code dans une branche match, vous pouvez utiliser des accolades. Par exemple, le code suivant imprimerait "Penny porte-bonheur !" chaque fois que la méthode est appelée avec un `Coin::Penny`, mais retournerait toujours la dernière valeur du bloc, `1` :

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny porte-bonheur !");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```