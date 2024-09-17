# Human Utils

These programs replace the standard UNIX commands for working with files: `touch`, `mkdir`, `mv`, `cp`, `rm`. The new utils are designed for interactive use and to prevent frustration and loss of data.

The new utils are `new` to create files and directories, `mov`e, `cop`y and `del`ete.

See [Three letters are too long](#three-letters-are-too-long) for shorter names and replacing the standard UNIX commands.

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
| `touch hockey`  | `new hockey` | <code style="color: green">N root/hockey</code> |
| `mkdir rocks`<br/>`touch rocks/ruby` | `new rocks/ruby` | <code style="color: green">N root/rocks/ruby</code>    |
| `mkdir rocks`<br/>`echo 💎 > rocks/ruby`     | `new rocks/ruby -- 💎`     | <code style="color: green">N root/rocks/ruby</code><br />&nbsp;&nbsp;`text: 💎` |
| `mkdir rocks` | `new rocks/` | <code style="color: green">N root/rocks/</code> |
| `mkdir rocks`<br />`cd rocks` | `c rocks`[*](#change-directory-integration) | <code style="color: green">N root/rocks/</code><br />&nbsp;&nbsp;`current working directory` |
| `mv rugby dogs` | `mov rugby dogs/`  | <code style="color: purple">M rugby -> dogs/rugby</code>  |
| `mv rugby tennis`  | `mov rugby tennis`<br />`... [y/N]?` <kbd>Enter</kbd> | <code style="color: red">D tennis</code><br /><code style="color: purple">M rugby -> tennis</code> |
| `rm -r dogs`<br />`mv cats dogs`  | `mov cats dogs`<br />`... [y/N]?` <kbd>Enter</kbd> | <code style="color: red">D dogs</code><br /><code style="color: purple">M cats -> dogs</code> |
| `mv rugby tennis dogs` | `mov rugby tennis dogs/` | <code style="color: purple">M rugby -> dogs/rugby</code><br/><code style="color: purple">M tennis -> dogs/tennis</code> |
| `mkdir sports`<br/>  `mv rugby tennis sports` | `mov rugby tennis sports/` | <code style="color: purple">R rugby -> sports/rugby</code><br/><code style="color: purple">R tennis -> sports/tennis</code> |
| `rm plants` | `del plants`<br />`... [y/N]?` <kbd>Enter</kbd> | <code style="color: red">D plants</code> |

## Principles

**Ask for confirmation before any irreversible action by default**
Irreversible actions include:

- deleting a file or a directory
- overwriting a file with different contents
- overwriting a directory with different files contents

**Do not change behavior based on the current state of the file tree**
The UNIX `mv a b` command performs a very different operation based on whether `b` does not exist or is a directory or a file (a move or a rename with a possible overwrite). In `human-utils` you instead choose which operation to perform (either via the path separator suffix or via explicit options).

**Create directories on demand**
If a directory is needed to perform any command, but it doesn't exist, it will get created. This applies to multiple nested directories as well. The behavior is similar to running the UNIX command `mkdir -p` with the right argument before every operation.

## Commands

### `new`

Creates one or more files or directories.

|Basic file creation|
|---|
|`new F` creates a file at the relative path `F`.|
|`new --file F` creates a file at the relative path `F`.|
|`new --file F/` errors out. File names cannot end in path separators.|


|Basic directory creation|
|---|
|`new D/` creates a directory at the relative path `D`.|
|`new --directory D/` creates a directory at the relative path `D`.|
|`new --directory D` creates a directory at the relative path `D`.|
|`-d` is a shortcut for `--directory`|


|Creating multiple files/directories|
|---|
|`new F1 F2 D1/ D2/` creates files at `F1` and `F2` and directories at `D1` and `D2`.|
|`new --file F --directory D` creates a file at `F` and a directory at `D`.|
|`new F -d D` creates a file at `F` and a directory at `D`.|
|`new --directory D1 --directory D2` creates directories at `D1` and `D2`.|
|`new D/{F1,F2,F3}` creates files at `D/F1`, `D/F2` and `D/F3`.|

|Initializing files with text|
|---|
|`new F -- hello world` creates a file at the relative path `F` with the UTF-8 string `hello world\n`.|
|`new F1 F2 -- hello` creates files at `F1` and `F2` with the UTF-8 string `hello\n`.|


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

## Change directory integration

It's convenient to both create a directory and navigate to it. You can set this up in your
shell. The examples below use the command alias `c`.

Fish:

```fish
function c -d "quickly cd and create directory if needed" -w cd
  new -d $argv
  and cd $argv
end
```