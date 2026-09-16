# Documentation

The reference documentation for Wickra Compile lives at
**[compile.wickra.org](https://compile.wickra.org)** — quickstarts, the API
surface for every binding, and the guides.

What stays here, beside the code, is the material that only makes sense next to
the implementation and has to change in the same commit as it:

- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`COMPILESPEC.md`](COMPILESPEC.md)
- [`Cookbook.md`](Cookbook.md)
- [`DETERMINISM.md`](DETERMINISM.md)
- [`TARGETS.md`](TARGETS.md)
- [`TEMPLATES.md`](TEMPLATES.md)

The per-binding READMEs under `bindings/` describe each package as its registry
page shows it.

## Editing the docs

The documentation site is a separate git repository at
`https://github.com/wickra-lib/wickra-compile-site`. Open a pull request there to
propose changes; the site is built with VitePress and deploys to
`compile.wickra.org`. The files in this directory change in the same commit as
the code they describe.
