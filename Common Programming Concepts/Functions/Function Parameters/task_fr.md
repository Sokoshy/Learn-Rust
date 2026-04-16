### Paramètres de fonction

Les fonctions peuvent également être définies pour avoir des _paramètres_, qui sont des variables spéciales faisant partie de la signature d’une fonction. Lorsqu'une fonction a des paramètres, vous pouvez lui fournir des valeurs concrètes pour ces paramètres. Techniquement, les valeurs concrètes sont appelées _arguments_, mais dans la conversation courante, on a tendance à utiliser les mots _paramètre_ et _argument_ de manière interchangeable, que ce soit pour les variables dans la définition d'une fonction ou pour les valeurs concrètes passées lors de l'appel d'une fonction.

La version réécrite suivante de `another_function` montre à quoi ressemblent les paramètres en Rust :

```rust
   fn main() {
       another_function(5);
   }
   
   fn another_function(x: i32) {
       println!("La valeur de x est : {}", x);
   }
```   

Essayez d'exécuter ce programme ; vous devriez obtenir la sortie suivante :

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
        Running `target/debug/functions`
   La valeur de x est : 5
```

La déclaration de `another_function` comporte un paramètre nommé `x`. Le type de `x` est spécifié comme étant `i32`. Lorsque `5` est passé à `another_function`, la macro `println!` place `5` à l'endroit où les accolades se trouvaient dans la chaîne de formatage.

Dans les signatures de fonction, vous _devez_ déclarer le type de chaque paramètre. C'est une décision délibérée dans la conception de Rust : exiger des annotations de type dans les définitions de fonction signifie que le compilateur n'a presque jamais besoin que vous les utilisiez ailleurs dans le code pour comprendre ce que vous voulez dire.

Lorsque vous souhaitez qu'une fonction ait plusieurs paramètres, séparez les déclarations de paramètres par des virgules, comme ceci :

```rust
   fn main() {
       another_function(5, 6);
   }
   
   fn another_function(x: i32, y: i32) {
       println!("La valeur de x est : {}", x);
       println!("La valeur de y est : {}", y);
   }
```   
Cet exemple crée une fonction avec deux paramètres, qui sont tous deux de type `i32`. La fonction affiche ensuite les valeurs de ses deux paramètres. Notez que les paramètres de fonction n'ont pas tous besoin d'être du même type, ils le sont simplement dans cet exemple.

Essayons d'exécuter ce code. Remplacez le programme actuel dans le fichier src/main.rs de votre projet functions par l'exemple précédent et exécutez-le :

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
        Running `target/debug/functions`
   La valeur de x est : 5
   La valeur de y est : 6
```

Parce que nous avons appelé la fonction avec `5` comme valeur pour `x` et `6` a été passé comme valeur pour `y`, les deux chaînes sont imprimées avec ces valeurs.

_Vous pouvez vous référer au chapitre suivant dans le livre The Rust Programming Language : [Comment fonctionnent les fonctions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)_