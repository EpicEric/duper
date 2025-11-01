# Contributing

Hello, and thank you for taking the time to read the contributing guidelines for Duper! Please read through our [Code of Conduct](./CODE_OF_CONDUCT.md) as well.

Here are some ways to contribute to the project:

## Creating issues

Please search the [existing issues](https://github.com/EpicEric/duper/issues) for answers or previous discussions before creating your own issue.

When contributing to a specific module in this project, include its name in the title of your issue. For example: "@duper-js/wasm: Add tests"

## Submitting changes

In the case that you'd like to make contributions to Duper, create an issue first if one does not exist. Also include the name of the improved module if applicable, for example: "@duper-js/wasm: Add tests"

If you wish to contribute changes to Duper, please [fork the repository](https://github.com/EpicEric/duper/fork), push your modifications to a branch, and create a [pull request](https://github.com/EpicEric/duper/compare). Make sure to [link to the original issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword) in your PR's body.

If possible, make sure that your changes pass all tests and linting/formatting checks before creating a pull request by running `just test` and `just lint`, respectively. This should ensure that your PR will pass the CI pipeline. These commands require the following tools to be installed in your system:

- [just](https://github.com/casey/just)
- [Rust >=1.88](https://rustup.rs/)
- [uv](https://docs.astral.sh/uv/) (for Python tests)
- [Node.js >=20](https://github.com/nvm-sh/nvm)
- [wasm-bindgen-cli](https://github.com/wasm-bindgen/wasm-bindgen/) (for WASM tests)

Please add a short description of any user-facing changes to the top of the CHANGELOG.md of the appropriate module(s), under the "Unreleased" section (or create one if it does not exist). The changelog should adhere to [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and it must emphasize any breaking changes.
