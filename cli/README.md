# subxt-cli

Utilities for working with substrate metadata for `subxt`

```
USAGE:
subxt <SUBCOMMAND>

FLAGS:
-h, --help
Prints help information

    -V, --version
            Prints version information


SUBCOMMANDS:
codegen     Generate runtime API client code from metadata
help        Prints this message or the help of the given subcommand(s)
metadata    Download metadata from a substrate node, for use with `subxt` codegen
```

## Metadata

Use to download metadata for inspection, or use in the `subxt` macro. e.g.

`subxt metadata -f bytes > metadata.scale`

```
USAGE:
    subxt metadata [OPTIONS]

OPTIONS:
    -f, --format <format>    the format of the metadata to display: `json`, `hex` or `bytes` [default: json]
        --url <url>          the url of the substrate node to query for metadata [default: http://localhost:9933]
```

## Codegen

Use to invoke the `subxt-codegen` crate which is used by `subxt-macro` to generate the runtime API and types. Useful
for troubleshooting codegen as an alternative to `cargo expand`, and also provides the possibility to customize the
generated code if the macro does not produce the desired API. e.g.

`subxt codegen | rustfmt --edition=2018 --emit=stdout`

```
USAGE:
    subxt codegen [OPTIONS]

OPTIONS:
    -f, --file <file>
            the path to the encoded metadata file

        --url <url>
            the url of the substrate node to query for metadata for codegen

```

