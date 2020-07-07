# Markdown Style Decisions

## Style Guidelines

### Fix All `markdownlint` Warnings

Markdown is used across this repository for documentation. These documents
should always pass all configured [`markdownlint`][1] checks. These checks
can be run on the fly by a [VS Code plugin][2], or on the command line with
[`markdownlint-cli`][github-markdownlint-cli].

Using `dwtj_rules_markdown`, `markdownlint` is automatically run during a
`bazel build` of `markdown_library` targets. If `markdownlint` finds a lint
warning, it will become a build error.

The style guidelines below are ones not checked by `markdownlint`.

### Prefer Markdown Link References For Long Links

If links are shorter than a couple of words or about 12 characters, it is ok to
use inline links. If a link is any longer, prefer to use [Markdown link
references][3]. (The syntax for link references is demonstrated in this
document.)

### Put Link Reference Definitions At The End

Link reference definitions should be placed at the end of the file after three
dashes forming a single [thematic break][4].

### Link Label Naming: Prefer Counting Numbers

For short documents, prefer to use the counting numbers as link labels.

This preference can be broken for documents which repeatly reference some
links. The named links should be defined in their own list above the numbered
links. (This syntax is demonstrated in this document.)

### Prefer Rooted Repository References

When using markdown to refer to files within this repository, prefer to use
fully qualified links rooted at the repository root. For example, [this link][5]
is a fully qualified link to this file.

Note that with VS Code opened at the repository root, one can follow such
links to the linked file.

### Prefer to Follow The Google Markdown Style Guide

The current Google Markdown Style Guide is available [here][6].

### Use the `.md` File Extension

Don't use `.markdown` or something else. Always use the `.md` file name suffix.

## Automated Style Checks

We use [`dwtj_rules_markdown`][7] to automatically run
[`markdownlint-cli`][github-markdownlint-cli] over all `markdown_library`
targets during `bazel build`. To temporarily disable this behavior locally,
comment out this line in the root `.bazelrc` file:

```bazelrc
build --aspects @dwtj_rules_markdown//markdown:defs.bzl%markdownlint_aspect
```

These rules are currently configured to find and use a `markdownlint` executable
on the system `PATH`. (See the `WORKSPACE` file.) Such an executable can be
installed using NPM:

```sh
npm install -g markdownlint-cli
```

---

[github-markdownlint-cli]: https://github.com/igorshubovych/markdownlint-cli

[1]: https://github.com/DavidAnson/markdownlint
[2]: https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint
[3]: https://spec.commonmark.org/0.28/#link-reference-definitions
[4]: https://spec.commonmark.org/0.28/#thematic-break
[5]: /architecture/decisions/style/markdown.md
[6]: https://github.com/google/styleguide/blob/505ba68c74eb97e6966f60907ce893001bedc706/docguide/style.md
[7]: https://github.com/dwtj/dwtj_rules_markdown
