# subxt-historic

**This crate is a work in progress and currently is released only as a preview.**

While `subxt` is a library for working at the head of a chain (submitting transactions and obtaining the current state), `subxt-historic` is a library for decoding blocks and state that are anywhere in a chain. To broadly summarize the differences:

| Feature                                 | subxt                        | subxt-historic                |
|-----------------------------------------|------------------------------|-------------------------------|
| Block access                            | Head of chain                | Any block in chain            |
| Connection to chain                     | Light client or RPC node     | Archive RPC nodes only        |
| Transaction submission                  | Yes                          | No                            |
| Metadata compatibility                  | V14 and newer                | Any version                   |

# Examples

See the [examples](https://github.com/paritytech/subxt/tree/master/historic/examples) folder for examples of how to use `subxt-historic`.
