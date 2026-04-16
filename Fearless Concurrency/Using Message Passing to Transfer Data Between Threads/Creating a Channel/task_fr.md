### Création d'un canal

Tout d'abord, dans l'exemple ci-dessous, nous allons créer un canal mais ne rien en faire pour l'instant. Notez que cela ne se compilera pas encore, car Rust ne sait pas quel type de valeurs nous voulons envoyer par le canal.

```rust
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();
    }
```

##### Créer un canal et assigner les deux moitiés à tx et rx

Nous créons un nouveau canal en utilisant la fonction `mpsc::channel`; `mpsc` signifie _multiple producteurs, un seul consommateur_. En bref, la manière dont la bibliothèque standard de Rust implémente les canaux signifie qu'un canal peut avoir plusieurs extrémités _émettrices_ qui produisent des valeurs mais une seule extrémité _réceptrice_ qui consomme ces valeurs. Imaginez plusieurs ruisseaux se rejoignant pour former une grande rivière : tout ce qui est envoyé dans l'un des ruisseaux finira dans une rivière unique à la fin. Nous commencerons par un seul producteur pour l'instant, mais nous ajouterons plusieurs producteurs lorsque cet exemple fonctionnera.

La fonction `mpsc::channel` renvoie un tuple, dont le premier élément est l'extrémité émettrice et le second élément est l'extrémité réceptrice. Les abréviations `tx` et `rx` sont traditionnellement utilisées dans de nombreux domaines pour _transmetteur_ et _récepteur_ respectivement, donc nous nommons nos variables ainsi pour indiquer chaque extrémité. Nous utilisons une instruction `let` avec un motif qui déstructure les tuples; nous aborderons l'utilisation des motifs dans les instructions `let` et la déstructuration au chapitre 18. Utiliser une instruction `let` de cette manière est une approche pratique pour extraire les éléments du tuple retourné par `mpsc::channel`.