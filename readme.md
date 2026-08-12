<h1 align="center">BetterTimer</h1>

BetterTimer is a small CLI, similar to the `sleep`, but with extra features:
  - progress bar with timer
  - notifications with the option to modify the message and name

## Usage

```sh
timer <duration>                      # 2ns, 42us, 5ms, 10s, 50m, 9h, etc
timer <duration> --name <name>        # Specify the name in notification ("BetterTimer" by default)
timer <duration> --message <message>  # Specify the message in notification ("Time's up" by default)
timer <duration> --silent             # Suppress notifications (silent mode)
```

Examples:
```sh
timer 5s --silent                                     # Or just `timer 5s -s`
timer 20m --name "Timer Demo" --message "Time to go!" # Or just `timer 20m -n "Timer Demo" -m "Time to go!"`
```

## Installation

WARNING: AUR and nixpkgs packages are planned

### Linux

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/clorine-software/bettertimer/releases/download/v1.0.5/bettertimer-installer.sh | sh
```

Or download manually

### Windows

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/clorine-software/bettertimer/releases/download/v1.0.5/bettertimer-installer.ps1 | iex"
```

Or download manually

### MacOS

Download manually


