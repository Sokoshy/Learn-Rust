### La durée de vie statique

Une durée de vie spéciale que nous devons aborder est `'static`, ce qui signifie que cette référence *peut* vivre pendant toute la durée du programme. Tous les littéraux de chaîne de caractères ont la durée de vie `'static`, que nous pouvons annoter comme suit :

```rust
let s: &'static str = "J'ai une durée de vie statique.";
```

Le texte de cette chaîne est stocké directement dans le binaire du programme, qui est toujours disponible. Par conséquent, la durée de vie de tous les littéraux de chaîne de caractères est `'static`.

Vous pourriez voir des suggestions d'utiliser la durée de vie `'static` dans les messages d'erreur. Mais avant de spécifier `'static` comme durée de vie pour une référence, réfléchissez à savoir si la référence que vous avez vit réellement pendant toute la durée de votre programme ou non. Vous pourriez vous demander si vous souhaitez qu'elle vive aussi longtemps, même si elle le pourrait. La plupart du temps, le problème résulte d'une tentative de créer une référence pendante ou d'un décalage entre les durées de vie disponibles. Dans de tels cas, la solution consiste à corriger ces problèmes, et non à spécifier la durée de vie `'static`.