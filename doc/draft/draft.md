vfs.apply(CopyOperation)
vfs.apply(FsOperation)
trait VirtualFsOperation.revert(&mut vfs) -> FsOperation


code ou mettre les opérations fs IO
code ou mettre les opérations virtuelles
code ou mettre l'inverse des operations virtuelles
code ou mettre l'inverse des operations d'io ? ( gérer les not recoverable )

trait Real -> need RealFileSystem
trait Virtual -> need Vfs
trait Write -> need writable FileSystem
trait Reverse -> need writable fileSystem
trait Read -> need readable FileSystem


execute(&vfs)

vfs.copy -> Copy
vfs.crate -> Create
vfs.remove -> Remove
.reverse() -> ReverseCopy
.reverse() -> ReverseCreate
.reverse() -> ReverseRemove



execute(&vfs)

read_dir -> ReadDir
stat -> Stat

s: VirtualStatus::([A-Za-z]+)\(([a-z_]+)
r: VirtualStatus::new(VirtualState::$1, $2



    //Synchronize approach is the best in regards of providing best effort against unwatched filesystem changes
    //Have to output a generator of operations over the fs
    //Next try with :
    // - Timestamps
    // - Vfs file source references
    // - Vfs source tracking
    // - Real time application of operations over filesystem
    // - Generate operations in order to let guard decision
    // To represent :
    // Deletion has to happen _in order_ because it have warrant about directory's children unicity : it frees a name's slot in a directory
    // Copy also have that warraant : it _reserve_ a name's slot in a directory
    // Move combines both : it frees a name's slot in a directory and reserve one into another OR THE SAME !