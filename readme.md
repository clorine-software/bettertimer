<h1 align="center">BetterTimer</h1>

BetterTimer is a small CLI, similar to the `sleep`, but with extra features:
  - progress bar with timer
  - named timers with message
  - notifications with the option to modify the message and name
  - countdown mode

## Usage

```sh
bettertimer <duration>                      # 2ns, 42us, 5ms, 10s, 50m, 9h30m10s, etc
bettertimer <duration> --name <name>        # Specify the name in notification ("BetterTimer" by default)
bettertimer <duration> --message <message>  # Specify the message in notification ("Time's up" by default)
bettertimer <duration> --silent             # Suppress notifications (silent mode)
bettertimer <duration> --countdown          # Countdown mode
```

Examples:
```sh
bettertimer 5s --silent --countdown                         # Or just `bettertimer 5s -s -c`
bettertimer 20m --name "Timer Demo" --message "Time to go!" # Or just `bettertimer 20m -n "Timer Demo" -m "Time to go!"`
```

## Installation

WARNING: AUR and nixpkgs packages are planned

### Linux

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/clorine-software/bettertimer/releases/download/v1.0.7/bettertimer-installer.sh | sh
```

Or download manually

### Windows

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/clorine-software/bettertimer/releases/download/v1.0.7/bettertimer-installer.ps1 | iex"
```

Or download manually

### MacOS

Download manually


