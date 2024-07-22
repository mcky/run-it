<div align=right>Table of Contents↗️</div>

<h1 align=center><code>just</code></h1>

<!-- <div align=center>
  <a href=https://crates.io/crates/PKG>
    <img src=https://img.shields.io/crates/v/PKG.svg alt="crates.io version">
  </a>
  <a href=https://github.com/mcky/run-it/actions>
    <img src=https://github.com/mcky/run-it/actions/workflows/ci.yaml/badge.svg alt="build status">
  </a>
  <a href=https://github.com/mcky/run-it/releases>
    <img src=https://img.shields.io/github/downloads/mcky/run-it/total.svg alt=downloads>
  </a>
</div> -->
<br>

`run-it` allows you to drop into any project, install dependencies and run tasks, without needing to worry about scripting and build tools.

<!-- ![screenshot](https://raw.githubusercontent.com/mcky/run-it/master/screenshot.png) -->

`run-it` will infer the correct tooling for your repository. Currently supported tools for task running are:

- make (`Makefile`)
- mise (`mise.toml`)
- just (`justfile`)
- pnpm (`package.json` scripts)
- yarn (`package.json` scripts)
- npm (`package.json` scripts)
- turborepo (`turbo.json`)

You can then run them with `run-it run <task>`:

```sh
$ cat Makefile
───────┬────────────────────────────────────
       │ File: Makefile
───────┼────────────────────────────────────
   1   │ test:
   2   │     cargo test
───────┴────────────────────────────────────
$ run-it run test
running: make test
---
running 1 test
test tools::tests::all_tools_are_matched ... ok
```
