/*
ROOT
    A (Dossier)
    B
    C
    D
        J
            F (Fichier)
        K
    E

ON ASSUME QU'ON NE GERE PAS LA CONCURRENCE ( édition du fs simultanément a l'utilisation de futurecommander )
DADOU => LA LECTURE DOIT ETRE FAITE UNE FOIS AU DEBUT AU DEMARRAGE DE L'APPLICATION
TUX => TROP DE META DONNEES POTENTIELLES => TROP DE LECTURE DISQUE !
Solution :
    Représente en mémoire :
        - Les nodes affectés par des opérations (a prevoir une possible serialization)
        - Tout leur parent recursivement jusqu'a la root
    Si l'utilisateur visualise un node qui n'est pas stocké en mémoire => fallback sur le système de fichier

Mais pour le MVP :
    - stocker tout en mémoire au démarrage
    - potentiellement lancer le logiciel "chrooté" dans un sous-dossier
    - faire des bench
    - faire une configuration minimale

Point du jour session 7
    - recupérer un noeud vide
    - faire une interface

"shell first"
    -> commencer l'implémentation du shell
*/

//preview extends std::fs
//override read_dir() -> comportement de preview
/*
DRAFT
We need to represents the children of a node because we need to display ordered list with ls
Ls prends un Path en argument
Node doit être capable de représenter un arbre de donnée
Node + Path => Node ( enfant )
*/

// TODO
// https://doc.rust-lang.org/std/fs/struct.ReadDir.html
// pub struct ReadDirBridge<'a> {}
// ReadDirBridge<'a> => NodeIterator<'a>

// TODO ABSTRAIRE L'OUTPUT
// Définir l'abstraction d'UI => trait / générique

// TODO ABSTRAIRE L'INPUT
// Définir l'abstraction pour wrapper les ReadDir et les DirEntry
// https://doc.rust-lang.org/std/os/unix/fs/trait.DirEntryExt.html
// futurecommander::ReadDirExt
// futurecommander::DirEntryExt

// Discuter de std::path::Path comme type natif ou outer edge