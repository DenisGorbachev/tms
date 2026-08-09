# Project

## Definitions

### struct MessageCommand

- Must have fields:
  - `subcommand: MessageSubcommand`

### struct InsertMessageCommand

- Must have fields:
  - `text_string: String`
  - `text_file: PathBuf`
  - `locators: Vec<Locator>`
  - `references: Vec<MessageId>`
  - `dependencies: Vec<MessageId>`
  - `dependents: Vec<MessageId>`
- Must have methods:
  - `run`
    - `let text_file = /* read text_file */`
    - `let text_stdin = /* read stdin */`
    - `let text = [text_string, text_file, text_stdin].map(str::trim).filter(not(str::is_empty)).join("\n\n")`
    - Must push `inserted_message_id` into every dependent

### struct UpdateMessageCommand

- Must have fields:
  - `id: MessageId`
  - `active: Option<bool>`

### struct ListMessageCommand

- Must have fields:
  - `active: Option<bool>`
  - `format: Format` (default: YAML)
- Must have methods:
  - `run`
    - Must apply filter fields that are `Some`
    - Must 

### type Locator

TODO:

- Define the type itself
- Think how to deal with changing locators due to file edits
  - Options
    - Require a commit with a message
- Think how to deal with unspecific locators (e.g. substring within a paragraph).
  - Options
    - Split down to sentences, use a sentence index.
- Think what exactly to store: names or indexes?
  - List items can only be identified by indexes
  - Expressions within functions can only be identified by indexes
  - Files can only be identified by names (paths)
  - Code items may be identified by either names or paths
- Think how to build locators via search with autocomplete
  - Options
    - A command for a search
    - Pipe all locators into fzf
      - But won't be convenient if locators contain indexes
        - Accept the reality that a single verbose locator maps to multiple precise locators?
- Decide whether it must include the repo URL

See also:

- [GitCommitScopedLocator](#struct-gitcommitscopedlocator)

### struct GitCommitScopedLocator

- Must have fields:
  - `commit: GitCommitHash`
  - `locator: ProjectScopedLocator`

### enum ProjectScopedLocator

- Must have variants:
  - `FileLineRange(FileLineRangeLocator)`
  - `RustPath(RustPathLocator)`

TODO:

- Define other variants

### struct Messages

- Must have methods:
  - `insert`
  - `update`
- Must pass requirements:
  - Every mutator that modifies `Message::dependencies` must validate that there are no cycles
