# TODO: Refactor
import time
from display import *
from inventory import *
#Functions for various events
def fairy_event(castle, player, castle_info):
    castle[player.movement_coordinate] = player.encountered_object
    display_array(castle = castle, game_sequence = 'non-battle', player = player)
    print('You encountered a fairy!')
    new_line(1)
    if all([player.hp == 100, player.mana == 100]):
        print('Your HP and mana are already full. Come back later.')
        time.sleep(1.5)
    else:
        if player.hp == 100 & player.mana < 100:
            print('Your mana was fully restored!')
            player.mana = 100
        elif player.hp < 100 & player.mana == 100:
            print('Your HP was fully restored!')
            player.hp = 100
        else:
            player.mana = 100
            player.hp = 100
        #Changing dictionary number to zero to prevent this event from activating again
        castle_info[player.movement_coordinate][1] = 0
    time.sleep(1)
    #Adding back door
    castle[player.movement_coordinate] = u'\U0001f6aa'
    return castle, player, castle_info

if __name__ == '__main__':
    print("You must run 'python3 start_game.py' to play Castle Descent.")   