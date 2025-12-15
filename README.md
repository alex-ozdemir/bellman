# bellman [![Crates.io](https://img.shields.io/crates/v/bellman.svg)](https://crates.io/crates/bellman) #

`bellman` is a crate for building zk-SNARK circuits. It provides circuit traits
and primitive structures, as well as basic gadget implementations such as
booleans and number abstractions.

`bellman` uses the `ff` and `group` crates to build circuits generically over a
scalar field type, which is used as the "word" of a circuit. Arithmetic
operations modulo the scalar field's prime are efficient, while other operations
(such as boolean logic) are implemented using these words.

## Roadmap

Currently `bellman` bundles an implementation of the Groth16 proving system.
This will be moved into a separate crate in the future, and `bellman` will
contain any utilities that make implementing proving systems easier.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Mirage+

This particular branch of bellman implements Mirage+, a commit-carrying zkSNARK
for interactive proofs. See:

* the [`crate::cc`](./src/mirage/mod.rs) module for the programming interface
* the [`crate::mirage`](./src/lib/mirage/mod.rs) module for the proof system
* the `permutation_arg_one_block_one_coin` unit test for an example of a zkSNARK
   for IPs
* the CirC compiler's bindings to Mirage illustrate how to use it as a CC-zkSNARK
   for IPs. [link](https://github.com/circify/circ/blob/291af0bf48897ba347329d2b7dc662369dd968af/src/target/r1cs/mirage.rs#L399)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
