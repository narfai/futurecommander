Garder la hiérarchie ou pas ? Descendante ou montante ?
Path ou PathBuf ?
Un BTree avec les NodeIndex des dossiers en clé ?

Parent NodeIndex -> Child NodeIndex




## Virtual versionning file systems

https://github.com/git-lfs/git-lfs/blob/master/docs/spec.md

https://code.fb.com/core-data/scaling-mercurial-at-facebook/

> Notify changes ( e.g hgwatchman ) ? distributed cache of content ( e.g remotefilelog ) ?


## Lowcost graph based data representations in Rust

### Graph Indices

https://docs.rs/petgraph/0.4.13/petgraph/
http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

### Vector Indices

https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
https://github.com/saschagrunert/indextree


## Programs written in rust which handle fs aspects

https://github.com/kamiyaa/joshuto
https://github.com/sharkdp/fd


## Rust Language

https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html
https://doc.rust-lang.org/nightly/edition-guide/rust-2018
https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
https://doc.rust-lang.org/reference/index.html
https://doc.rust-lang.org/rust-by-example/index.html
https://github.com/mre/idiomatic-rust


## Shells

https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
https://doc.rust-lang.org/std/process/struct.Command.html


## Iterators

https://burgers.io/wrapped-iterators-in-rust
https://burgers.io/extending-iterator-trait-in-rust
https://medium.com/@jordan_98525/reference-iterators-in-rust-5603a51b5192
https://doc.rust-lang.org/stable/std/iter/
https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
https://stackoverflow.com/questions/32682876/is-there-any-way-to-return-a-reference-to-a-variable-created-in-a-function

> extending an iterator !! https://beyermatthias.de/blog/2018/02/02/extending-an-iterator-in-rust/

## Referentiation

https://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/

## TUI / GUI

https://crates.io/crates/cursive
https://github.com/maps4print/azul / https://azul.rs/

## OO Pattern

State Pattern => https://doc.rust-lang.org/1.30.0/book/second-edition/ch17-03-oo-design-patterns.html
Adapter Pattern => https://github.com/jdavis/rust-design-patterns/blob/master/patterns/adapter.rs

## Bitbucket pipelining

build artifacts : https://confluence.atlassian.com/bitbucket/deploy-build-artifacts-to-bitbucket-downloads-872124574.html
pipeline yml format : https://confluence.atlassian.com/bitbucket/configure-bitbucket-pipelines-yml-792298910.html

## Generics ( Sized Monomorphisation ) against traits ( Unsized )

https://stackoverflow.com/questions/24635146/can-this-code-be-written-without-generics?rq=1

## crates

CLI testing :  https://crates.io/crates/assert_cmd
https://rust-lang-nursery.github.io/cli-wg/index.html

## Rendering ( & gaming )

http://arewegameyet.com/

## Threading

https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa

## Rust and nodejs

https://neon-bindings.com/
https://nwjs.io/
https://proton-native.js.org/#/

## Rust lints

https://github.com/rust-lang/rust-clippy

## QA

https://github.com/rust-lang/miri#bugs-found-by-miri

## Functional programming

https://www.amazon.com/Hands-Functional-Programming-Rust-applications/dp/1788839358/ref=as_li_ss_tl?ie=UTF8&linkCode=sl1&tag=tutorialedge-20&linkId=3664a43d9e5e1a441704bdce071983df

## Portable Native Client for Rust ( PNaCl )
https://internals.rust-lang.org/t/pnacl-support-for-rust/888


## Code Coverage

https://axiomatic.neophilus.net/code-coverage-in-rust/
https://github.com/xd009642/tarpaulin

## Weird Archlinux problem with crt2.o for windows target ( mingw64 )

https://github.com/rust-lang/rust/issues/48272

## Rust with Nix \o/

https://docs.rs/nix/0.13.0/nix/


## CI
https://buildkite.com/features
https://codeship.com/pricing
https://buddy.works/pricing
https://circleci.com/pricing/
pro> https://semaphoreci.com/pricing

## CD
https://concourse-ci.org/
https://www.spinnaker.io/concepts/

## Serialize, Deserialize
https://github.com/dtolnay/typetag
https://serde.rs/

## An OS in rust ( seriously )
https://os.phil-opp.com/

## Http server
https://actix.rs/

## wasm
https://rustwasm.github.io/wasm-bindgen/reference
https://nodejs.org/api/child_process.html
https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html

## thread about running cross platform process from javascript

https://stackoverflow.com/questions/26924209/node-js-child-process-doesnt-work-in-node-webkit

## binary encoding / decoding

https://peteris.rocks/blog/serialize-any-object-to-a-binary-format-in-rust/
https://github.com/TyOverby/bincode

## Webstorm & nwjs

thanks ! https://stackoverflow.com/questions/35677387/nw-js-and-webstorm

## Icon font

https://cloudfour.com/thinks/seriously-dont-use-icon-fonts/

## CSS tips

http://tachyons.io/
https://mithril.js.org/css.html

## Tokio

https://blog.passcod.name/2018/mar/07/writing-servers-with-tokio

## CSS tips

http://tachyons.io/
https://mithril.js.org/css.html

## Node Stream

https://www.freecodecamp.org/news/node-js-streams-everything-you-need-to-know-c9141306be93/

## GRPC

https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o

## Rust + webassembly to make webapp

https://seed-rs.org
