## 用 Rust 实现俄罗斯方块游戏
master 分支打算改为使用 bevy 实现。
之前使用了 rust-sdl2 来实现，放在 sdl2 分支。
## Build
### MAC:
```
brew install sdl2 sdl2_mixer sdl2_image sdl2_ttf
LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib" cargo build
```

![](./image/screenshot.png)