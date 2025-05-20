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

| coreutils                                    | human-utils                                           | outcome                                                                                                                     |
| -------------------------------------------- | ----------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `touch hockey`                               | `new hockey`                                          | <code style="color: green">N hockey</code>                                                                                  |
| `mkdir rocks`<br/>`touch rocks/ruby`         | `new rocks/ruby`                                      | <code style="color: green">N rocks/ruby</code>                                                                              |
| `mkdir rocks`<br/>`echo 💎 > rocks/ruby`     | `new rocks/ruby -- 💎`                                | <code style="color: green">N rocks/ruby</code><br />&nbsp;&nbsp;text: `💎`                                                  |
| `mkdir rocks`                                | `new rocks/`                                          | <code style="color: green">N rocks/</code>                                                                                  |
| `mkdir rocks`<br />`cd rocks`                | `c rocks`[\*](#change-directory-integration)          | <code style="color: green">N rocks/</code><br />&nbsp;&nbsp;current working directory                                       |
| `mv rugby dogs`                              | `mov rugby dogs/`                                     | <code style="color: purple">M rugby -> dogs/rugby</code>                                                                    |
| `mv rugby tennis`                            | `mov rugby tennis`<br />`... [y/N]?` <kbd>Enter</kbd> | <code style="color: red">D tennis</code><br /><code style="color: purple">M rugby -> tennis</code>                          |
| `rm -r dogs`<br />`mv cats dogs`             | `mov cats dogs`<br />`... [y/N]?` <kbd>Enter</kbd>    | <code style="color: red">D dogs</code><br /><code style="color: purple">M cats -> dogs</code>                               |
| `mv rugby tennis dogs`                       | `mov rugby tennis dogs/`                              | <code style="color: purple">M rugby -> dogs/rugby</code><br/><code style="color: purple">M tennis -> dogs/tennis</code>     |
| `mkdir sports`<br/> `mv rugby tennis sports` | `mov rugby tennis sports/`                            | <code style="color: purple">R rugby -> sports/rugby</code><br/><code style="color: purple">R tennis -> sports/tennis</code> |
| `rm plants`                                  | `del plants`<br />`... [y/N]?` <kbd>Enter</kbd>       | <code style="color: red">D plants</code>                                                                                    |

## Principles

### By default ask for confirmation before any irreversible action

Irreversible actions include:

- deleting a file or a directory
- overwriting a file with different contents
- overwriting a directory with different files contents

### Do not change behavior based on the current state of the file tree

The UNIX `mv a b` command performs a very different operation based on whether `b` does not exist or is a directory or a file (a move or a rename with a possible overwrite). In `human-utils` you instead choose which operation to perform (either via the path separator suffix or via explicit options).

The only change in behavior based on the state of the file tree in `human-utils` is whether you will be asked to proceed or not.

### Create directories on demand

If a directory is needed to perform any command, but it doesn't exist, it will be created. This applies to multiple nested directories as well. The behavior is similar to running the UNIX command `mkdir -p` with the appropriate argument before every operation.

### Be consistent

The move and copy commands' behaviors should be closely related: a `mv` should behave like a `cp` followed by `rm` of the original file(s), yet the UNIX interface for these commands differs (for example, `cp` changes behavior based on a trailing path separator on source, but `mv` doesn't). `human-utils` instead maintain a consistent interface between the two commands.

## Commands

### `new`

Creates one or more files or directories.

| Basic file creation                                                   |
| --------------------------------------------------------------------- |
| `new F` creates a file at the relative path `F`.                      |
| `new --file F` creates a file at the relative path `F`.               |
| `new --file F/` errors out. File names cannot end in path separators. |

| Basic directory creation                                           |
| ------------------------------------------------------------------ |
| `new D/` creates a directory at the relative path `D`.             |
| `new --directory D/` creates a directory at the relative path `D`. |
| `new --directory D` creates a directory at the relative path `D`.  |
| `-d` is a shortcut for `--directory`                               |

| Creating multiple files/directories                                                  |
| ------------------------------------------------------------------------------------ |
| `new F1 F2 D1/ D2/` creates files at `F1` and `F2` and directories at `D1` and `D2`. |
| `new --file F --directory D` creates a file at `F` and a directory at `D`.           |
| `new F -d D` creates a file at `F` and a directory at `D`.                           |
| `new --directory D1 --directory D2` creates directories at `D1` and `D2`.            |
| `new D/{F1,F2,F3}` creates files at `D/F1`, `D/F2` and `D/F3`.                       |

| Initializing files with text                                                                          |
| ----------------------------------------------------------------------------------------------------- |
| `new F -- hello world` creates a file at the relative path `F` with the UTF-8 string `hello world\n`. |
| `new F1 F2 -- hello` creates files at `F1` and `F2` with the UTF-8 string `hello\n`.                  |
| `new F -- ''` asks for a confirmation if a non-empty file exists at `F` and erases it.                |

| Creating parent directories                                                 |
| --------------------------------------------------------------------------- |
| `new D1/D2/D3/` creates directories at `D1`, `D1/D2` and `D1/D2/D3`.        |
| `new D1/D2/F` creates directories at `D1`, `D1/D2` and a file at `D1/D2/F`. |

## Three letters are too long

You can make aliases to these utils and even shadow existing UNIX commands. To avoid unexpected behavior in existing scripts, create the aliases only in interactive sessions.

For example in Fish shell `config.fish`:

```fish
if status is-interactive
    # alias ne new
    alias mv mov
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
    set cd_output (cd $argv 2>&1)
    if test $status -neq 0
        if test (count $argv) -eq 1 && not test -e $argv[1]
            new -d $argv[1]
            cd $argv[1]
        else
            echo $cd_output
        end
    end
end
```

## Shell integration

These commands can change the state of your file system. They are built to be used from an interactive shell. Interactive shells have the concept of the "current working directory" - the directory from which you're executing a command. Yet in most shells, commands cannot change the working directory. For this reason, there are two command invocations that require a shell integration:

1. `mov . /some/where/else` - In this case we would obviously want the current working directory to follow the move.
2. `del .` - In this case we would like the working directory to move up the file tree.

Both scenarios must work when the first argument points to any ancestor of the current working directory.

This can be achieved with the `--track-cwd-change <file_path>` option, like this:

```fish
function mv -w mov
    set tracking_file /tmp/__mov_cwd_change__
    mov --track-cwd-change $tracking_file $argv
    if test -f $tracking_file
        set new_cwd (cat $tracking_file)
        /bin/rm $tracking_file
        cd $new_cwd
    end
end

function rm -w del
    set tracking_file /tmp/__del_cwd_change__
    del --track-cwd-change $tracking_file $argv
    if test -f $tracking_file
        set new_cwd (cat $tracking_file)
        /bin/rm $tracking_file
        cd $new_cwd
    end
end
```
