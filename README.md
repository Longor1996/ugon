# UGON

A **object-notation** / data-format / binary-encoding that uses **gaps** in the **UTF-8 encoding**
as tags for schemaless data: **Utf-8 Gap-based Object Notation** → ***UGON***!

> This repository contains the in-development version of the UGON specification and implementation.

```txt
Creation Date:  2022-07-29
Format Name:    UTF-8 Gap-based Object Notation (UGON)
Format Type:    Data Interchange / Serialization Format
Format Ext.:    .ugon
Format MIME:    application/ugon
Magic Bytes:    TBD
Specification:  https://github.com/Longor1996/ugon/SPEC.md
Implementation: https://github.com/Longor1996/ugon (Rust)
```

## Quickstart

**TODO:** Publish to https://crates.io/ once stable enough.

**TODO:** Use GitHub Actions to render the spec to HTML for GitHub Pages.

**TODO:** Use GitHub Actions for pre-built tool artefacts.

## Objectives

**TODO:** Outline general objectives. (Roadmap?)

The objectives (/goals) of this format are as follows:

- **Be self-describing / schemaless:**  
  Blobs of ugon-encoded data can *always* be read by any spec-compliant decoder,
  regardless by what application originally produced the data.  
  What the data actually *is*, is, of course, application-defined.
- **Decoding is *straight-forward* and *fast*:**  
  A decoder should only need one byte of look-ahead (i.e.: `.peek()`)
  to know what, and for continuous data, how much, to read next.
- **Compact data representation:**  
  Primitive values/arrays can be (mem-)copied as-is,
  often occurring values are folded into tags,
  and dynamic structures take as few bytes as reasonable.

## Roadmap

The highest level of tasks to be done for 1.0.0 to be reached.

- [ ] Finish the first draft of the specification.
- [ ] Implement the decoder according to the spec.
- [ ] Implement the encoder according to the spec.
- [ ] Implement the [serde](https://serde.rs/) feature.
- [ ] First release & publishing to [crates.io](https://crates.io/).
- [ ] Make the library compatible with WASM.
- [ ] Create a GitHub-pages site having...
  - [ ] ...a rendered version of the current spec.
  - [ ] ...a usable JSON-to-UGON converter.
  - [ ] ...a web-viewer for UGON files.
- [ ] Create a command-line application that...
  - [ ] ...can convert JSON to UGON.
  - [ ] ...can convert UGON to JSON.
  - [ ] ...can print a tree-view of a given UGON file.
  - [ ] ...can return subsets of a given UGON file via 'path' semantics.
- [ ] Create a visual editor for UGON files.
  - Almost certainly using [egui](https://github.com/emilk/egui).

## How and why?

**TODO:** Explain why (ref: https://xkcd.com/927/).

**TODO:** Explain how UTF-8 gap encoding works and why it's safe.

**TODO:** Explain the various types of tags.

## Get Involved

Documentation, bug reports, pull requests and all other contributions are welcome!

Please note that any contribution submitted to this repository (and it's metadata)
must be provided under the same [licensing terms](./LICENSE).

## References

- https://en.wikipedia.org/wiki/UTF-8#Encoding
