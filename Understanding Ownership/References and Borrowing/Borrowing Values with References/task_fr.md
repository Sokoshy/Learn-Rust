## Emprunter des valeurs avec des références

Le problème avec le code de tuple dans la tâche précédente est que nous devons retourner le `String` à la fonction appelante pour pouvoir encore utiliser le `String` après l'appel à `calculate_length` car le `String` a été déplacé dans `calculate_length`.

Voici comment vous définiriez et utiliseriez une fonction `calculate_length` qui a une référence à un objet en paramètre au lieu de prendre possession de la valeur :

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("La longueur de '{}' est {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Tout d'abord, remarquez que tout le code de tuple dans la déclaration de variable et la valeur de retour de la fonction a disparu. Ensuite, notez que nous passons `&s1` dans `calculate_length` et, dans sa définition, nous prenons `&String` plutôt que `String`.

Ces esperluettes sont des _références_, et elles vous permettent de vous référer à une certaine valeur sans en prendre possession. La figure 5 montre un diagramme.

<img alt="&amp;String s pointant vers String s1" src="https://doc.rust-lang.org/stable/book/img/trpl04-05.svg" class="center">

##### Figure 5: Un diagramme de &String s pointant vers String s1

> Note : Le contraire de faire référence en utilisant `&` est la _déréférencement_, qui se fait avec l'opérateur de déréférencement, `*`. Nous verrons quelques utilisations de l'opérateur de déréférencement au chapitre 8 et discuterons des détails du déréférencement au chapitre 15.

Regardons de plus près l'appel de la fonction ici :

```rust
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
```

La syntaxe `&s1` nous permet de créer une référence qui _se réfère_ à la valeur de `s1` mais ne la possède pas. Parce qu'elle ne la possède pas, la valeur à laquelle elle fait référence ne sera pas supprimée lorsque la référence sortira de sa portée.

De même, la signature de la fonction utilise `&` pour indiquer que le type du paramètre `s` est une référence. Ajoutons quelques annotations explicatives :

```rust
fn calculate_length(s: &String) -> usize { // s est une référence à un String
    s.len()
} // Ici, s sort de sa portée. Mais comme il n'a pas la possession de ce à quoi
  // il se réfère, rien ne se passe.
```

La portée dans laquelle la variable `s` est valide est la même que celle de tout paramètre de fonction, mais nous ne supprimons pas ce à quoi la référence pointe lorsqu'elle sort de sa portée car nous n'avons pas possession. Lorsque des fonctions ont des références comme paramètres au lieu des valeurs réelles, nous n'aurons pas besoin de renvoyer les valeurs pour redonner possession car nous ne l'avions jamais.

Nous appelons le fait d'avoir des références comme paramètres de fonction _emprunt_. Comme dans la vie réelle, si une personne possède quelque chose, vous pouvez l'emprunter. Quand vous avez fini, vous devez le lui rendre.