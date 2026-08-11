<h1 align="center">BetterTimer</h1>

BetterTimer is a small CLI, similar to the `sleep`, but with extra features:
  - progress bar with timer
  - notifications with the option to modify the message

## Usage

```sh
timer <duration> # 2ns, 42us, 5ms, 10s, 50m, 9h, etc
timer <duration> --message <message> # Specify the message in notification
timer <duration> --silent # BetterTimer will not show notification
```

Examples:
```sh
timer 5s --silent # Or just `timer 5s -s`
timer 20m --message "Time to go!" # Or just `timer 20m -m "Time to go!"`; "Time's up" by default
```
