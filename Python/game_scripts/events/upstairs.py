import time
from display import *


def upstairs_event(castle, player):
    display_array(castle=castle, game_sequence="non-battle", player=player)
    print("You already came from upstairs.")
    time.sleep(1)


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
