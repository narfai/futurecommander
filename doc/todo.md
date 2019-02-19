# TODOs

## Feature

* gérer le move OK
* mkdir OK
* touch OK
* tree
* edit ( store file modification files into tmp space )
* apply des opérations ( déduire les opérations des delta )
* shell auto-complete

## Bugs

* Dangling virtual cwd

## Tests

* tests add / sub / add / sub
* tests sur la lecture fs d'un fichier copié ou déplacé
* tests sur une arborescence de profondeur 100+
* tests dir / file interversion Af->Cf / Bd -> Ad / Cf -> Bf
* windows testing
* benchmarks

## Optimization performances and readability

* .is_virtual() optimization
* find a way that vdelta + or += plays with references
* proper to_state, as_state
* Vpath.path => Vpath.identity OK
* Vdelta += Vdelta
* VirtualPath slices : Actuellement les PathBuf et Vpath sont clonés A CHAQUE FOIS
* interfaçage propre en Result, Attach, Detach, Read, Rm, Copy, Move
* Retourner dans l'objet de retour le FS READ COUNT
* proper logs & errors
* handle unwrap
* Gestion de PathId pour indexer les path en sized a travers l'ensemble du vfs ( et plus seulement les deltas )
* split up tests
* better API
* makes the shell supports event-based reactions


## Known limitations

* If user modify the actual fs while using this soft, consistency is not preserved => fill the tree with notify, or use optimizer & representations to replay operations with best effort
* Do NOT supports symlinks ( because compatibility and problem's coming from graph cyclic behaviors )
