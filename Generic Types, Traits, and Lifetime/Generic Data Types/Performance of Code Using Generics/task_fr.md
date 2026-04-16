### Performance du code utilisant des génériques

Vous vous demandez peut-être s'il y a un coût d'exécution lorsque vous utilisez des paramètres de type générique. La bonne nouvelle est que Rust implémente les génériques de telle manière que votre code ne s'exécute pas plus lentement en utilisant des types génériques qu'il ne le ferait avec des types concrets.

Rust accomplit cela en effectuant la monomorphisation du code qui utilise des génériques au moment de la compilation. La *monomorphisation* est le processus qui consiste à transformer le code générique en code spécifique en remplissant les types concrets qui sont utilisés lors de la compilation.

Dans ce processus, le compilateur fait l'inverse des étapes que nous avons utilisées pour créer la fonction générique dans le troisième extrait de code de cette section : le compilateur regarde tous les endroits où le code générique est appelé et génère du code pour les types concrets avec lesquels le code générique est appelé.

Voyons comment cela fonctionne avec un exemple qui utilise l'énumération `Option<T>` de la bibliothèque standard :

```rust
let integer = Some(5);
let float = Some(5.0);
```

Lorsque Rust compile ce code, il effectue la monomorphisation. Pendant ce processus, le compilateur lit les valeurs qui ont été utilisées dans les instances de `Option<T>` et identifie deux types de `Option<T>`: l'un est `i32` et l'autre est `f64`. Ainsi, il étend la définition générique de `Option<T>` en `Option_i32` et `Option_f64`, remplaçant ainsi la définition générique par des définitions spécifiques.

La version monomorphisée du code ressemble à ce qui suit. L'`Option<T>` générique est remplacée par les définitions spécifiques créées par le compilateur :

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Parce que Rust compile le code générique en code qui spécifie le type dans chaque instance, nous ne payons pas de coût d'exécution pour l'utilisation des génériques. Lorsque le code s'exécute, il fonctionne exactement comme il le ferait si nous avions dupliqué chaque définition à la main. Le processus de monomorphisation rend les génériques de Rust extrêmement efficaces à l'exécution.