# Human Utils

These programs replace the standard UNIX commands for working with files: `mv`, `cp`, `rm`, `touch`, `mkdir`. The new utils are designed for interactive use and to prevent frustration and loss of data.

The new utils are `nef` new file, `ned` new directory, `mov`e, `nam`e, `cop`y, `rem`ove and `undo`.

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
| `touch hockey`  | `nef hockey` | <code style="color: green">A root/hockey</code> |
| `mkdir rocks`<br/>`touch rocks/ruby` | `nef rocks/ruby` | <code style="color: green">A root/rocks/ruby</code>    |
| `mkdir rocks`<br/>`echo 💎 > rocks/ruby`     | `nef rocks/ruby 💎`     | <code style="color: green">A root/rocks/ruby</code><br />&nbsp;&nbsp;`text: 💎` |
| `mv rugby dogs` | `mov rugby dogs`  | <code style="color: purple">R rugby -> dogs/rugby</code>  |
| `mv rugby tennis`  | `nam rugby tennis`<br />`... [y/N]?` <kbd>Enter</kbd> | <code style="color: red">D tennis</code><br /><code style="color: purple">R rugby -> tennis</code> |
| `mv rugby tennis dogs` | `mov rugby tennis dogs` | <code style="color: purple">R rugby -> dogs/rugby</code><br/><code style="color: purple">R tennis -> dogs/tennis</code> |
| `mkdir sports`<br/>  `mv rugby tennis sports` | `mov rugby tennis sports` | <code style="color: purple">R rugby -> sports/rugby</code><br/><code style="color: purple">R tennis -> sports/tennis</code> |

## Principles

**Ask for confirmation before any irreversible action by default**
Irreversible actions include:

- deleting a file or a directory
- overwriting a file with different contents
- overwriting a directory with different files contents

**Do not change behavior based on the current state of the file tree**
The UNIX `mv a b` command performs a very different operation based on whether `b` is an existing directory or not (a move or a rename with a possible overwrite). In `human-utils` you instead choose which operation to perform (by choosing `mov` or `nam`).

## Three letters are too long

You can make aliases to these utils. To avoid unexpected behavior in existing scripts, create the aliases only in interactive sessions.

In Fish shell this can be accomplished by amending the `fish_greeting` function:

```
function fish_greeting
    alias nf nef
    alias nd ned
    alias mv mov
    alias nm nam
    alias cp cop
    alias rm rem
end
```
