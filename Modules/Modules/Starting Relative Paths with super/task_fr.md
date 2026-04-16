### Commencer les chemins relatifs avec super

Nous pouvons construire des chemins relatifs qui commencent dans le module parent, plutôt que dans le module actuel ou à la racine de la crate, en utilisant `super` au début du chemin. Cela revient à commencer un chemin de système de fichiers avec la syntaxe `..`. Utiliser `super` nous permet de référencer un élément que nous savons être dans le module parent, ce qui peut faciliter la réorganisation de l'arborescence des modules lorsque le module est étroitement lié au parent, mais que le parent pourrait être déplacé ailleurs dans l'arborescence des modules un jour.

Considérez le code ci-dessous qui représente une situation où un chef corrige une commande incorrecte et l'apporte personnellement au client. La fonction `fix_incorrect_order` définie dans le module `back_of_house` appelle la fonction `deliver_order` définie dans le module parent en spécifiant le chemin vers `deliver_order` en commençant par `super` :

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

##### Appeler une fonction en utilisant un chemin relatif commençant par `super`

La fonction `fix_incorrect_order` est dans le module `back_of_house`, donc nous pouvons utiliser `super` pour remonter au module parent de `back_of_house`, qui dans ce cas est `crate`, la racine. De là, nous cherchons `deliver_order` et nous le trouvons. Succès ! Nous pensons que le module `back_of_house` et la fonction `deliver_order` sont susceptibles de rester dans la même relation l'un par rapport à l'autre et d'être déplacés ensemble si nous décidons de réorganiser l'arborescence des modules de la crate. Par conséquent, nous avons utilisé `super` de sorte qu’il y aura moins d’endroits à mettre à jour dans le code à l’avenir si celui-ci est déplacé dans un autre module.