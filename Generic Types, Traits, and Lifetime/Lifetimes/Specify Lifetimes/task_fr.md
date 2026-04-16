## Spécifier les durées de vie

Le compilateur Rust a besoin de savoir comment vérifier si les références fournies sont valides pour pouvoir informer le programmeur si une référence risque de sortir de sa portée avant d'être utilisée. Rappelez-vous, les références sont des emprunts et ne possèdent pas leurs propres données. Que se passe-t-il si leur propriétaire sort de la portée ?

<div class="hint">
Laissez le compilateur vous guider.

De plus, jetez un œil au [livre](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) si vous avez besoin d'aide.
</div>