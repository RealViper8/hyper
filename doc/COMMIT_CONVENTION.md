# Commit Message Guidelines

Thank you for contributing to **Hyper**! Please follow these guidelines to keep our commit history clean and readable.

You can start the message after the prefix with either an uppercase or a lowercase letter, and do not put a period at the end of the commit message.

## Allowed Prefixes

* **`feat:`** Adding a new feature or capability.
* **`fix:`** fixing a bug.
* **`docs:`** Updating or adding documentation.
* **`style:`** Code formatting changes that do not affect the code logic.
* **`refactor:`** Restructuring code without changing its external behavior or adding features.
* **`test:`** adding or updating tests.
* **`ci:`** Continuous integration — GitHub Actions workflows, smoke programs under `ci/`, or other automation that runs on push/PR.
* **`chore:`** Routine tasks, maintenance, or dependency updates.

## One commit per file

Do **not** bundle many unrelated file changes into a single commit with one message.

- Prefer **one commit per changed file**, each with its own matching prefix and message.
- A PR may contain several commits; that is expected and preferred over one large catch-all commit.

### Exceptions

A single commit may touch more than one file only when those files are an **inseparable** unit of work, for example:

- Renaming or moving a file and updating the imports/paths that must change with it
- Adding a CI fixture (`.hyp`) and the matching workflow step that runs it
- A tiny paired change where splitting would leave the tree broken between commits (e.g. a new symbol in one module and its only call site)

If files are only loosely related, still split them into separate commits.

If you encounter any difficulties writing commits with prefixes or want to suggest new prefixes outside of the existing ones, please open an [Issues](https://github.com/Yusupov-Muhammadyusuf/hyperlang/issues) to discuss it.
