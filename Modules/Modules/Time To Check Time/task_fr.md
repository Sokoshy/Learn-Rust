## Il est temps de vérifier l'heure

Vous pouvez utiliser le mot-clé `use` pour importer des chemins de modules depuis n'importe où et en particulier depuis la bibliothèque standard de Rust dans votre portée. Importez `SystemTime` et `UNIX_EPOCH` depuis le module `std::time`. Points de style bonus si vous pouvez le faire en une seule ligne !

Faites compiler le code !

<div class="hint">

`UNIX_EPOCH` et `SystemTime` sont déclarés dans le module `std::time`. Ajoutez une instruction `use` pour les importer dans la portée. Vous pouvez utiliser des chemins imbriqués ou l'opérateur `glob` pour les importer en utilisant seulement une ligne.

</div>