# TODOs ( replaced by https://trello.com/b/A2BvQdR9/futurecommander )

## Feature

* gérer le move OK
* mkdir OK
* touch OK
* trim les inputs OK
* tree
* edit ( store file modification files into tmp space )
* apply des opérations ( déduire les opérations des delta )
* shell auto-complete
* handle file renaming
* shell errors
* handle real vfserrors instead of panic!
* support des .. et .
* symlink guard
* handle mes misuse et afficher l'usage au lieu de crash

## Dev / ops Conveniences

* contributing.md
    * doc about git branching
* readme.md
* doc comments
* dockerization ( + local build windows )
* proper taskboard / milestone schema

## Bugs

* Dangling virtual cwd
* tree / mv A B / tree
* mv B A / mv A/B/D A ( file_system.rs ligne 177 ) OK
* mkdir E / mv E A / ls E OK
* cp A A/B => shouldn't be allowed
* mv B B/D => shouldn't be allowed
* remove cwd or ancestors shouldn't be allowed

## GUI

neon + nwjs + mithril + bulma ? makes it easily remote server file explorer on web through an API ?

## Tests

* tests add / sub / add / sub OK
* tests sur la lecture fs d'un fichier copié ou déplacé
* tests sur une arborescence de profondeur 100+
* tests dir / file interversion Af->Cf / Bd -> Ad / Cf -> Bf OK
* windows testing ~OK
* benchmarks
* tests sur le support de la root
* status / create / remove / copy all UC
* chaos monkey which generates *a lot* of random operations to makes some errors bubblings

## Optimization performances and readability

* proper to_state, as_state OK
* Vpath.path => Vpath.identity OK
* Vdelta += Vdelta
* interfaçage propre en Result, Attach, Detach, Read, Rm, Copy, Move => see State Pattern
* Retourner dans l'objet de retour le FS READ COUNT
* proper logs & errors
* handle unwrap
* split up tests OK
* better API OK
* shell supports piping and sub thread / child
* stream outputs & reads

## Known limitations

* If user modify the actual fs while using this soft, consistency is not preserved => fill the tree with notify, or use optimizer & representations to replay operations with best effort
* Do NOT supports symlinks ( because compatibility and problem's coming from graph cyclic behaviors )



