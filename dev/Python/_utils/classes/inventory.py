import copy, random, time, numpy as np, msvcrt
from display import *
from item_description import item_description
from Python.game_scripts.classes.cursor import cursor

# TODO: Recreate inventory function to a class
class Inventory:
    def __init__(self):
        # Items
        self.items = {
            "crystal_ball": 0,
            "bento": 0,
            "magnifying_glass": 0,
            "test_tube": 0
            }

        # Unicode
        self.unicode = {
            "crystal_ball": "\U0001F52E",
            "bento": "\U0001F371",
            "magnifying_glass": "\U0001F50E",
            "test_tube": "\U0001F9EA"
            }
    

def inventory(player,game_sequence):
    cursor.position = 0
    player_action = ''
    object = copy.deepcopy(player.observable_item_inventory[1])
    while not player_action == 'e':
        new_line(50)
        print('Inventory')
        print('___________________________________________')
        new_line(1)
        print(''.join(['{}'.format(cell + ' ') for cell in player.observable_item_inventory])) 
        new_line(1)
        print('___________________________________________')
        #Item description
        item_description(object = object, cost = 'no')
        print('___________________________________________')
        new_line(1)
        if not object == '':     
            new_line(1)
            print(f'Number in inventory: {player.hidden_item_inventory[object]}')
            new_line(1)
        while not player_action in ['a','d','s','e']:
            print('a(left), d(right), s(select), e(exit): ')
            player_action = msvcrt.getch().decode('utf-8').lower()
        if player_action in ['a', 'd']:
            player = cursor.move_cursor(object = player, player_action = player_action, game_sequence = 'inventory')
            player_action = ''
        if player_action == 's':
            if any([game_sequence == 'free movement', game_sequence == 'battle' and object in [u'\U0001F371',u'\U0001F9EA']]):
                player = use_item(player = player,object = object)
                if 0 in player.hidden_item_inventory.values():
                    player.reset_inventory()
                player_action = ''
            elif object != '':
                new_line(50)
                print('Inventory')
                print('___________________________________________')
                new_line(1)
                print(''.join(['{}'.format(cell + ' ') for cell in player.observable_item_inventory])) 
                new_line(1)
                print('___________________________________________')
                new_line(1)
                print('Cannot use this item during battle.')
                player_action = ''
                time.sleep(1) 
            else:
                player_action = ''

        #Get object arrow is pointing to
        object = copy.deepcopy(player.observable_item_inventory[cursor.position + 1])
    #If loop is broken, reset cursor position
    #Erase cursor
    player.observable_item_inventory[np.where(player.observable_item_inventory == u'\u2771')[0][0]] = ''
    player.observable_item_inventory[0] = u'\u2771' 
    return player

def use_item(player, object):
    new_line(50)
    #Reduce number in inventory
    player.hidden_item_inventory[object] -= 1
    print('Inventory')
    print('___________________________________________')
    new_line(1)
    print(''.join(['{}'.format(cell + ' ') for cell in player.observable_item_inventory])) 
    new_line(1)
    print('___________________________________________')
    new_line(1)
    match object:
        case u'\U0001F52E': 
            player.zombie_halt = random.sample([num for num in range(10,21)],1)[0]
            print(f'Zombie halted for {player.zombie_halt} steps')
            print('Temporarily halts zombie movement.')
        case u'\U0001F371': 
            if player.hp == 100:
                print('You are already at full health.')
                #Add back item
                player.hidden_item_inventory[object] += 1
            elif player.hp + 20 > 100:
                restore = player.hp + 20 + (100 - (player.hp + 20))
                print(f'Your HP increased by {abs(100 - player.hp)} points.')
                player.hp = copy.deepcopy(restore)
            else:
                print('Your HP increased by 20 points.')
                player.hp += 20
        case u'\U0001F50E':
            player.monster_threshold = 0
            print('The door has been revealed.')
        case u'\U0001F9EA':
            if player.mana == 100:
                print('Your mana is already maxed out.')
                #Add back item
                player.hidden_item_inventory[object] += 1
            elif player.mana + 30 > 100:
                restore = player.mana + 30 + (100 - (player.mana + 30))
                print(f'Your mana increased by {abs(100 - player.mana)} points.')
                player.mana = copy.deepcopy(restore)
            else:
                print('Your mana increased by 30 points.')
                player.mana += 30

    time.sleep(1) 
    return player