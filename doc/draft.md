# Events

## Initialization

Will create empty VirtualFileSystem and OperationList.

## Read Event ( Er )

Specify a path.

Over the given path :
Will enrich the VFS with OperationList data if concerned.
Else will enrich the VFS with real FS data.

## ADD Event ( Ea )

Specify a source.
Specify a destination.

Will enrich the OperationList with a new Operation.

## DEL Event ( Ed )

Specify a path to delete.

Will enrich the OperationList with a new Operation.

## Apply Event (Eap)

Will consume OperationList LIFO by concrete operation applying.
Will write the FS indeed.

# Types

## VirtualFileSystem

Tree-ordered data which able to store hierarchic set of Nodes.

Could be enriched by :

* a real fs path.
* an Operation.

### FileNode

Contains a path.
Represents an fs file.

### DirectoryNode

Contains a path.
Represents an fs directory.

## OperationList

LIFO which contains ordered set of Operation.
Could be consumed atomically.

### DeleteOperation

Represents a non-applied delete operation over the FS.

This operation is deprecated if the given path does not exists.

### AddOperation

Represents a non-applied add operation over the FS.

This operation is deprecated if :

* the given source does not exists.
* OR the parent of destination does not exists.
