# 🌟 Catch the Falling Stars - Game Demo

## What You'll See When You Run The Game

### 1. Menu Screen
```
🌟 CATCH THE FALLING STARS! 🌟

Use ← → arrows to move your basket
Catch the falling stars to score points!
Don't let 10 stars fall or you lose!

Press SPACE to start
```

### 2. Game Screen
- Orange rectangular basket at the bottom (your player)
- Yellow square stars falling from the top
- Score counter in top-left: "Score: 0"
- Lives counter in top-right: "Lives: 3"

### 3. Gameplay Demo Sequence

**Starting the game:**
- Press SPACE from menu
- Your orange basket appears at bottom center
- First star spawns at random X position at top

**First 10 seconds:**
- Stars fall slowly (speed: 100 pixels/second)
- Spawn rate: 1 star per second
- Move left/right with arrow keys to catch stars
- Each caught star: +10 points, star disappears
- Each missed star: -1 life, star disappears at bottom

**After 50 points (5 stars caught):**
- Difficulty increases automatically
- Stars fall faster (speed: 120 pixels/second)
- Spawn rate increases (1.2 stars per second)

**Game progression:**
- Score: 0-49 → Level 1 (slow)
- Score: 50-99 → Level 2 (faster)
- Score: 100-149 → Level 3 (even faster)
- Continues scaling indefinitely

**Game Over scenario:**
- Miss 3 stars total → Lives reach 0
- Screen shows: "🌟 GAME OVER! 🌟"
- Displays final score
- Options: "Press R to restart" or "Press ESC to quit"

### 4. Visual Elements

**Colors:**
- Background: Black
- Player basket: Orange rectangle (80x20 pixels)
- Stars: Yellow squares (20x20 pixels)
- Score text: White
- Lives text: Red
- Title text: Gold
- Game over text: Red

**Movement:**
- Player moves smoothly left/right
- Player constrained to screen bounds
- Stars fall straight down
- Collision detection when player touches star

### 5. Expected Performance

**Frame rate:** 60 FPS
**Window size:** 800x600 pixels
**Response time:** Immediate input response
**Memory usage:** Minimal (cleans up entities properly)

### 6. Sound Effects (None - Visual Only Game)
This version focuses on visual gameplay without audio.

### 7. Typical Play Session

1. **Launch:** `cargo run`
2. **Menu:** Press SPACE
3. **Play:** Use arrows to catch 10-20 stars
4. **Challenge:** Difficulty ramps up noticeably
5. **End:** Miss 3 stars, see final score
6. **Restart:** Press R to play again

**Average first-time score:** 50-80 points
**Good score:** 150+ points
**Expert score:** 300+ points

### 8. Technical Demonstration

The game showcases:
- Bevy ECS (Entity Component System)
- State management (Menu → Playing → GameOver)
- Real-time input handling
- Collision detection
- Dynamic difficulty scaling
- Resource management
- Timer systems
- Random number generation
- UI rendering
- Entity cleanup between states

This demonstrates a complete, playable game loop with proper Bevy architecture patterns.