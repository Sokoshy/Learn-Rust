## Macros

Nous avons utilisé des macros comme `println!` tout au long de ce livre, mais nous n'avons pas pleinement exploré ce qu'est une macro et comment elle fonctionne. Le terme _macro_ se réfère à une famille de fonctionnalités dans Rust : les macros _déclaratives_ avec `macro_rules!` et trois sortes de macros _procédurales_ :

*   Les macros `#[derive]` personnalisées qui spécifient le code ajouté avec l'attribut `derive` utilisé sur les structs et les enums
*   Les macros de type attribut qui définissent des attributs personnalisés utilisables sur n'importe quel élément
*   Les macros de type fonction qui ressemblent à des appels de fonction mais qui opèrent sur les jetons spécifiés comme leurs arguments

Nous parlerons de chacune d'elles à tour de rôle, mais d'abord, examinons pourquoi nous avons même besoin de macros alors que nous avons déjà des fonctions.