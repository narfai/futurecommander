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
