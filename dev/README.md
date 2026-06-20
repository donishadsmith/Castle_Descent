# Castle Descent 

A roguelike game implemented in R Studio and Python, featuring Unicode-based graphics and turn-based
gameplay. The objective is to reach the final floor of a procedurally generated castle (represented as a 3D array)
while avoiding the zombie.

**The R Studio version is recommended.**

## Technologies Used

[![RStudio Community: RStudio IDE](https://img.shields.io/endpoint?url=https%3A%2F%2Frstudio.github.io%2Frstudio-shields%2Fcategory%2Frstudio-ide.json)](https://community.rstudio.com/c/rstudio-ide)
[![Python 3.10](https://img.shields.io/badge/python-3.10-blue.svg)](https://www.python.org/downloads/release/python-3100/)

## Requirements

#### Python (Windows only):
Relies on Windows-specific ``msvcrt`` module.

Requirements:
- Python 3.10 or higher
- ``numpy`` 

#### RStudio:
Requirements:

- RStudio
- ``rstudioapi`` package

## Starting Game

#### Python:

While in the same folder containing `start_game.py` in your Terminal, run:

```python
python3.10 start_game.py
```

#### RStudio:
1. Open `start_game.R` in RStudio
2. Set the working directory to match the source file location:
   - Go to: Session -> Set Working Directory -> To Source File Location
   - Or: To File Pane Location if the directory is visible in the File Pane
3. Run the following code:

```R 
source("game_scripts/create_environment.R")
```
## Gameplay

### Controls

Movement: `W` `A` `S` `D` keys

#### Features
The game world wraps around, allowing the player to move from one edge to the opposite side. Zombies use predictive
pathfinding and can move in 8 directions but cannot wrap around edges or pass through doors. 

#### Main Game View

<img src="https://user-images.githubusercontent.com/112973674/232951810-b99fda6f-8a2a-4799-81ee-abbc9c3fb4eb.png" width="600" alt="Main">


#### Combat System
Battle monsters to earn gold and random item drops.

<img src="https://user-images.githubusercontent.com/112973674/232952120-a4e15a95-f680-4b2f-ae2e-e31c5446a3db.png" width="600" alt="Combat">


#### Merchant Shop
Gold earned from battles can be used to purchase items from the merchant shop.

<img src="https://user-images.githubusercontent.com/112973674/232951879-da6dee34-49aa-4724-83e6-0fc20b733967.png" width="600" alt="Merchant">
render_grid(screen, "door", coord, cell_coord)
#### Special Encounters
While some doors may contain monsters and the exit to the next floor, others may contain:

- 🧞 Genies that boost your attack or reduce mana costs
- 🧚 Fairies that restore your health

<img src="https://user-images.githubusercontent.com/112973674/232952208-948873a4-7a48-4451-84b4-3b563827dbb6.png" width="600" alt="Genie">
