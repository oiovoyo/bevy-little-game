# 🌟 Catch the Falling Stars! 🌟

A fun little game built with Bevy where you catch falling stars with your basket!

## 🎮 How to Play

1. **Start the game**: Run `cargo run` and press **SPACE** to begin
2. **Move your basket**: Use **←** and **→** arrow keys to move left and right
3. **Catch the stars**: Move under the falling yellow stars to catch them
4. **Score points**: Each star you catch gives you 10 points
5. **Don't miss too many**: You lose a life when a star hits the ground
6. **Survive**: Game over when you run out of lives!

## 🎯 Game Features

- **Dynamic difficulty**: The game gets faster and spawns more stars as your score increases
- **Lives system**: You start with 3 lives, lose one for each missed star
- **Real-time scoring**: Watch your score climb as you catch stars
- **Smooth controls**: Responsive left/right movement
- **Visual feedback**: Colorful sprites and clear UI

## 🎨 Game Elements

- **🟠 Orange Rectangle**: Your basket (the player)
- **🟡 Yellow Squares**: Falling stars to catch
- **Score Display**: Top-left corner
- **Lives Counter**: Top-right corner

## 🎮 Controls

### Menu Screen
- **SPACE**: Start the game

### Playing
- **← (Left Arrow)**: Move basket left
- **→ (Right Arrow)**: Move basket right

### Game Over Screen
- **R**: Restart and return to menu
- **ESC**: Quit the game

## 🚀 Quick Start

```bash
cd bevy_hello_world
cargo run
```

## 🎯 Tips for High Scores

1. **Stay centered**: Position yourself in the middle to reach stars on both sides
2. **Watch the patterns**: Stars spawn randomly but you can anticipate clusters
3. **Don't panic**: The game speeds up, but smooth movements are better than frantic ones
4. **Practice timing**: Learn to position yourself under falling stars early

## 🔧 Technical Details

- **Engine**: Bevy 0.12.1
- **Language**: Rust
- **Window**: 800x600 pixels
- **Dependencies**: `bevy`, `rand`

## 🎪 Game States

1. **Menu**: Welcome screen with instructions
2. **Playing**: The main game loop
3. **Game Over**: Final score and restart options

Enjoy catching those stars! ⭐