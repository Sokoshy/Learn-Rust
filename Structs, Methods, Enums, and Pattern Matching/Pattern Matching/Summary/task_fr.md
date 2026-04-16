## Résumé

Nous avons maintenant vu comment utiliser les enums pour créer des types personnalisés pouvant être l'une des valeurs énumérées. Nous avons montré comment le type `Option<T>` de la bibliothèque standard vous aide à utiliser le système de types pour éviter les erreurs. Lorsque les valeurs d'un enum contiennent des données, vous pouvez utiliser `match` ou `if let` pour extraire et utiliser ces valeurs, en fonction du nombre de cas que vous devez traiter.

Vos programmes Rust peuvent désormais exprimer les concepts de votre domaine en utilisant des structs et des enums. Créer des types personnalisés pour utiliser dans votre API garantit la sécurité des types : le compilateur s'assurera que vos fonctions reçoivent uniquement les valeurs du type attendu par chaque fonction.

Afin de fournir à vos utilisateurs une API bien organisée, facile à utiliser et qui expose uniquement ce dont vos utilisateurs auront besoin, tournons-nous maintenant vers les modules de Rust.