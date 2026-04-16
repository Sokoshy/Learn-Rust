### Prévenir les références pendantes avec les durées de vie

Le principal objectif des durées de vie est de prévenir les références pendantes, qui amènent un programme à référencer des données autres que celles qu'il est censé référencer. Considérons le programme ci-dessous, qui comporte une portée externe et une portée interne.

```rust,ignore,does_not_compile
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
```

#### Une tentative d'utilisation d'une référence dont la valeur est sortie de la portée

> Remarque : Les exemples de cette liste et de plusieurs listes suivantes déclarent des variables sans leur attribuer une valeur initiale, de sorte que le nom de la variable existe dans la portée externe. À première vue, cela pourrait sembler être en contradiction avec le fait que Rust n'accepte pas de valeurs nulles. Cependant, si nous essayons d'utiliser une variable avant de lui donner une valeur, nous obtiendrons une erreur à la compilation, ce qui montre que Rust n'autorise effectivement pas les valeurs nulles.

La portée externe déclare une variable nommée `r` sans valeur initiale, et la portée interne déclare une variable nommée `x` avec la valeur initiale de 5. À l'intérieur de la portée interne, nous tentons d'attribuer la valeur de `r` comme une référence à `x`. Ensuite, la portée interne se termine et nous tentons d'afficher la valeur de `r`. Ce code ne se compilera pas parce que la valeur à laquelle `r` fait référence a quitté la portée avant que nous essayions de l'utiliser. Voici le message d'erreur :

```console
error[E0597]: `x` ne vit pas assez longtemps
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ la valeur empruntée ne vit pas assez longtemps
8  |         }
   |         - `x` libéré ici alors qu'il est encore emprunté
9  | 
10 |         println!("r: {}", r);
   |                           - emprunt utilisé ultérieurement ici
```

La variable `x` ne "vit pas assez longtemps". La raison est que `x` sera hors portée lorsque la portée interne se termine à la ligne 7. Mais `r` est toujours valide pour la portée externe ; comme sa portée est plus grande, on dit qu'il "vit plus longtemps". Si Rust permettait cet code de fonctionner, `r` référencerait une mémoire qui a été désallouée lorsque `x` est sorti de la portée, et toute tentative d'opération avec `r` ne fonctionnerait pas correctement. Alors, comment Rust détermine-t-il que ce code est invalide ? Il utilise un vérificateur d'emprunts.