## Unifier les durées de vie

Donc, si le compilateur ne fait que valider les références passées
aux paramètres annotés et au type de retour, qu'avons-nous besoin de changer ?

<div class="hint">
Rappelez-vous que la durée de vie générique <code>'a</code> recevra la 
durée de vie concrète qui est égale à la plus petite des durées de vie de <code>x</code> et <code>y</code>.
</div>

<div class="hint">
Vous pouvez emprunter au moins deux chemins pour atteindre le résultat souhaité tout en conservant le bloc interne :

1. Déplacer la déclaration de `string2` pour qu'elle vive aussi longtemps que `string1` (comment est déclaré `result` ?)
2. Déplacer `println!` dans le bloc interne.

</div>