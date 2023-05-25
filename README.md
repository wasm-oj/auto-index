# auto-index

Auto-Indexer for WOJ Problem Box

## Usage

Add `.github/workflows/auto-index.yml`:

```yml
# .github/workflows/auto-index.yml
name: Auto Index

on:
    push:
        branches:
            - main

permissions:
    contents: write

jobs:
    auto-index:
        runs-on: ubuntu-latest
        steps:
            - name: Checkout
              uses: actions/checkout@v3

            - name: Auto Index
              uses: wasm-oj/auto-index@main

            - name: Commit
              uses: EndBug/add-and-commit@v9
              with:
                  default_author: github_actions
                  message: "[skip ci] chore: update index"
                  push: true
```
