# Tik Tak Toe MiniMax Implementation
The following project is a terminal-based implementation of the game of Tik Tak Toe in Rust, as a beginer Rust project.
## 1. Minimax
For this minimax function, we will explore each move until each hypothetical game ends, if the game ends up in a win, that board has a score of 10 - the amount of moves that had to have been played. If the player looses, then the board gets a score of -10 + the amount of moves made. If there is a tie it gets a score of 0.
## 2. How to play
To play input the row and column index you want to place your piece in and press enter, then the bot makes its move if no one has already wone. If no player manages to make a wining move and all spots are taken, the game ends in a tie.
