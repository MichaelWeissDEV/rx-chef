# Shell completions and man page

The `completions` and `manpage` commands derive artifacts from the same Clap
command tree that powers `--help`. They do not depend on checked-in generated
files.

## Bash

```console
mkdir -p ~/.local/share/bash-completion/completions
rxchef completions bash \
  > ~/.local/share/bash-completion/completions/rxchef
```

Start a new shell or source the generated file.

## Zsh

```console
mkdir -p ~/.zfunc
rxchef completions zsh > ~/.zfunc/_rxchef
```

Ensure `~/.zfunc` is in `fpath` before `compinit`:

```zsh
fpath=(~/.zfunc $fpath)
autoload -Uz compinit && compinit
```

## Fish

```console
mkdir -p ~/.config/fish/completions
rxchef completions fish > ~/.config/fish/completions/rxchef.fish
```

Fish loads files in that directory automatically.

## PowerShell

```powershell
.\rxchef.exe completions powershell > rxchef-completion.ps1
. .\rxchef-completion.ps1
```

Add the dot-source line to the PowerShell profile after moving the generated
file to a stable per-user path.

## Elvish

```console
rxchef completions elvish > rxchef.elv
```

Source the file from the Elvish configuration according to the shell version in
use.

## Manual page

Write roff to stdout or an explicit file:

```console
rxchef manpage > rxchef.1
rxchef manpage --output rxchef.1
man ./rxchef.1
```

For a user installation on Linux/macOS:

```console
mkdir -p ~/.local/share/man/man1
install -m 644 rxchef.1 ~/.local/share/man/man1/rxchef.1
mandb ~/.local/share/man 2>/dev/null || true
```

The generated manual documents the complete command hierarchy and options for
the installed binary version. Regenerate it after upgrading rxchef; do not copy
one from a different release.

## Packaging rule

Release packages can generate these artifacts from the packaged binary during
assembly. Keeping Clap as the source of truth avoids drift between parser,
`--help`, shell completion, and the manual page.
