## Tester une fonction

Ce test ne teste pas notre fonction — modifiez-le pour qu'il le fasse de manière à ce que le test réussisse. Ensuite, écrivez une deuxième fonction de test appelée `is_false_when_odd` qui vérifie si nous obtenons le résultat attendu lorsque nous appelons `is_even(5)`.

<div class="hint">
  Vous pouvez appeler une fonction directement là où vous passez des arguments à <code>assert!</code> — vous pouvez donc faire quelque chose comme :

```rust
assert!(having_fun());
```

  Si vous voulez vérifier que vous obtenez bien false, vous pouvez inverser le résultat de ce que vous faites en utilisant `!`, comme :
  
```rust
assert!(!having_fun());
```
</div>