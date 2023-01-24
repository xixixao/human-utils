# Human Utils

These programs replace the standard UNIX commands for working with files: `touch`, `mkdir`, `mv`, `cp`, `rm`. The new utils are designed for interactive use and to prevent frustration and loss of data.

The new utils are `new` to create files and directories, `mov`e, `ren`ame, `cop`y, `del`ete and `undo`.

See [Three letters are too long](#three-letters-are-too-long) for shorter names.

## Comparison

Scenario:

```js
📂 root // <- current working directory
├ 📂 dogs
├ 📂 cats
├ 📂 plants
│ └ 📜 daisy // text: 🌼
├ 📜 rugby // text: 🏉
└ 📜 tennis // text: 🎾
```

<!-- prettier-ignore -->
| coreutils  | human-utils | outcome  |
| --- | --- | --- |
| `touch hockey`  | `fil hockey` | <code style="color: green">N root/hockey</code> |
| `mkdir rocks`<br/>`touch rocks/ruby` | `new rocks/ruby` | <code style="color: green">N root/rocks/ruby</code>    |
| `mkdir rocks`<br/>`echo 💎 > rocks/ruby`     | `new rocks/ruby 💎`     | <code style="color: green">N root/rocks/ruby</code><br />&nbsp;&nbsp;`text: 💎` |
| `mkdir rocks` | `new rocks/` | <code style="color: green">N root/rocks/</code> |
| `mkdir rocks`<br />`cd rocks` | `new rocks/ -c` | |
| `mv rugby dogs` | `mov rugby dogs`  | <code style="color: purple">R rugby -> dogs/rugby</code>  |
| `mv rugby tennis`  | `ren rugby tennis`<br />`... [y/N]?` <kbd>Enter</kbd> | <code style="color: red">D tennis</code><br /><code style="color: purple">R rugby -> tennis</code> |
| `mv rugby tennis dogs` | `mov rugby tennis dogs` | <code style="color: purple">R rugby -> dogs/rugby</code><br/><code style="color: purple">R tennis -> dogs/tennis</code> |
| `mkdir sports`<br/>  `mv rugby tennis sports` | `mov rugby tennis sports` | <code style="color: purple">R rugby -> sports/rugby</code><br/><code style="color: purple">R tennis -> sports/tennis</code> |

## Principles

**Ask for confirmation before any irreversible action by default**
Irreversible actions include:

- deleting a file or a directory
- overwriting a file with different contents
- overwriting a directory with different files contents

**Do not change behavior based on the current state of the file tree**
The UNIX `mv a b` command performs a very different operation based on whether `b` is an existing directory or not (a move or a rename with a possible overwrite). In `human-utils` you instead choose which operation to perform (by choosing `mov` or `ren`).

**Create directories on demand**
If a directory is needed to perform any command, but it doesn't exist, it will get created. This applies to multiple nested directories as well. The behavior is similar to running the UNIX command `mkdir -p` with the right argument before every operation.

## `undo`

The `undo` command offers to run a reverse of **only** the last performed operation, if it was reversible:

```sh
$ ren tennis t
R tennis t
$ undo
Run `ren t tennis` [Y/n]? yes
R t tennis
$ del tennis
D tennis
$ undo
Cannot undo `del tennis`.
```

It the last operation was partially irreversible (for example overwriting an existing file), `undo` will only reverse the reversible part.

## Three letters are too long

You can make aliases to these utils and even shadow existing UNIX commands. To avoid unexpected behavior in existing scripts, create the aliases only in interactive sessions.

In Fish shell this can be accomplished by amending the `fish_greeting` function:

```
function fish_greeting
    alias ne new
    alias mv mov
    alias re ren
    alias cp cop
    alias rm del # or `alias de del`
    # alias mkdir "new -d"
    # alias touch "new -f"
end
```
