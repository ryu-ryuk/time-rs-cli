<div align="center">
  <div style="display: inline-block; vertical-align: middle;">
    <img src="docs/time-rs-cli.png" alt="Time-RS CLI Logo" width="128" style="border-radius: 12px;"/>
  </div>
  <div style="display: inline-block; vertical-align: middle; margin-left: 16px;">
    <h1 style="margin: 0; padding: 0;">Time-RS CLI</h1>
  </div>
</div>


<h6 align="center" style="color:#bac2de;">
  A minimal Catppuccin-themed TUI countdown timer.
</h6>

<p align="center">
  <!-- GitHub Stars -->
  <a href="https://github.com/ryu-ryuk/time-rs-cli/stargazers">
    <img src="https://img.shields.io/badge/Stars-★%20%7C%20ryu--ryuk%2Ftime--rs--cli-cba6f7?style=for-the-badge&labelColor=1e1e2e&color=cba6f7&logo=github&logoColor=cdd6f4" alt="GitHub Stars"/>
  </a>
  <!-- GitHub Issues -->
  <a href="https://github.com/ryu-ryuk/time-rs-cli/issues">
    <img src="https://img.shields.io/badge/Issues-Open-f38ba8?style=for-the-badge&labelColor=1e1e2e&color=f38ba8&logo=github&logoColor=cdd6f4" alt="GitHub Issues"/>
  </a>
  <!-- License -->
  <a href="https://github.com/ryu-ryuk/time-rs-cli/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-89b4fa?style=for-the-badge&labelColor=1e1e2e&color=89b4fa&logo=openaccess&logoColor=cdd6f4" alt="MIT License"/>
  </a>
    <!-- AUR -->
  <a href="https://aur.archlinux.org/packages/timers">
    <img src="https://img.shields.io/badge/AUR-timers-89b4fa?style=for-the-badge&logo=arch-linux&logoColor=white&labelColor=1e1e2e" alt="AUR Timers"/>
  </a>
</p>


<p align="center">
  <!-- Rust Edition -->
  <img src="https://img.shields.io/badge/Rust-2024--edition-89b4fa?style=for-the-badge&logo=rust&logoColor=white&labelColor=1e1e2e&color=89b4fa" alt="Rust 2024 Edition"/>
  <!-- Ratatui -->
  <img src="https://img.shields.io/badge/ratatui-Terminal_UI-b4befe?style=for-the-badge&logo=gnome-terminal&logoColor=white&labelColor=1e1e2e&color=b4befe" alt="ratatui Terminal UI"/>
  <!-- Theme -->
  <img src="https://img.shields.io/badge/Theme-Catppuccin_Mocha-f5c2e7?style=for-the-badge&logo=palette&logoColor=white&labelColor=1e1e2e&color=f5c2e7" alt="Catppuccin Mocha Theme"/>
  <!-- Platform -->
  <img src="https://img.shields.io/badge/Platform-Portable_(Linux/Mac/WSL)-a6e3a1?style=for-the-badge&logo=linux&logoColor=white&labelColor=1e1e2e&color=a6e3a1" alt="Platform Portable"/>
</p>


<p align="center" style="color:#a6adc8; font-size: 14.5px; line-height: 1.6; max-width: 700px; margin: auto;">
  <strong style="color:#cdd6f4;">Time-RS CLI</strong> is a highly minimal, distraction-free terminal countdown timer.<br/>
  Built in <span style="color:#89b4fa;">Rust</span> using <span style="color:#b4befe;">ratatui</span> and themed with <span style="color:#f5c2e7;">Catppuccin Mocha</span>.<br/>
  Perfect for Pomodoros, build pauses, CLI workflows, or just flexing nerdy timers in your terminal.
</p>

---

## 🎥 Preview

<p align="center">
  <img src="docs/preview.gif" alt="Preview of Time-RS CLI" width="75%"/>
</p>

---



## 🧪 Installation


<a href="https://aur.archlinux.org/packages/timers">
  <img src="https://img.shields.io/badge/AUR-timers-89dceb?style=for-the-badge&logo=arch-linux&logoColor=white&labelColor=1e1e2e" alt="AUR Timers"/>
</a>


### 📦 Arch Linux / Manjaro (via AUR)

```sh
yay -S timers
```

>[!TIP]
> you can also use paru or any other AUR helper.

## ⏳ Features

* ⌨️ TUI controls:
  - `r` — restart timer
  - `j/k` — add/subtract 10s
  - `h` — show/hide help
  - `esc` — close help
  - `q` — quit
  - `p` - pomodoro timer

* 🎨 Catppuccin Mocha theming (colors, borders, text)
* 🧠 Smart redraws and minimalist centered layout
* 🧱 Built using [`ratatui`](https://github.com/ratatui-org/ratatui) + `crossterm`
* 📦 Single binary, zero dependencies at runtime



## 🧪 Try Kitty Popup Mode

You can use a floating terminal like `kitty` to simulate a popup view: | Might not work :(

```sh
kitty --override initial_window_width=50c \
      --override initial_window_height=8c \
      ./target/release/timers
```

## ⚙ Build 
```sh
git clone https://github.com/ryu-ryuk/time-rs-cli
cd time-rs-cli
cargo build --release
./target/release/timers
```

## 🌫 Contributing

I welcome contributions! Whether it's bug fixes, new features, or improvements, feel free to open issues or submit pull requests.

### Development Setup
 
* Fork the repository.

* Clone your fork.

* Create a new branch for your feature or bugfix.

* Make your changes and commit with clear messages.

* Push your branch and open a pull request.

#### Thank you for helping make Time-RS CLI better! 💜
