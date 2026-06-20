# TODO: Refactor
import copy, numpy as np, msvcrt
from Python.game_scripts.classes.cursor import cursor
from item_description import item_description
from display import new_line
class merchant_class:
    def __init__(self, item_costs, shop_inventory, selection_menu):
        self.item_costs = item_costs
        self.shop_inventory = shop_inventory
        self.selection_menu = selection_menu
        self.max_buy = None
    def menu(self, player, game_sequence):
        cursor.position = 0
        player_action = ''
        object = copy.deepcopy(self.shop_inventory[1])
        while not player_action == 'e':
            #Display shop
            new_line(50)
            print(u'\U0001F9DD')
            print('Merchant')
            print('___________________________________________')
            new_line(1)
            print(''.join(['{}'.format(cell + '  ') for cell in self.shop_inventory])) 
            new_line(1)
            print('___________________________________________')
            #Item description
            item_description(object = object, cost = 'yes')
            print('___________________________________________')
            new_line(1)
            print(f'\U0001F4B0: {player.money} | Number in inventory: {player.hidden_item_inventory[object]}')
            new_line(1)
            while not player_action in ['a','d','s','e']:
                print('a(left), d(right), s(select), e(exit): ')
                player_action = msvcrt.getch().decode('utf-8').lower()
            if player_action in ['a','d']:
                self.shop_inventory = cursor.move_cursor(object = self.shop_inventory, player_action = player_action, game_sequence = game_sequence) 
                object = copy.deepcopy(self.shop_inventory[cursor.position + 1])
                #Clear player action
                player_action = ''
            elif player_action == 's':
                if player.money >= self.item_costs[object]:
                    player = self.add_item(player = player, object = object)
                    player_action = ''
                else:
                    player_action = ''
        self.shop_inventory[cursor.position] = ''
        self.shop_inventory[0] = u'\u2771'
        return player
    #Selection menu to choose number of items to buy
    def add_item(self, player, object):
        player_action = ''
        #Getting max amount players can buy based on how much money the player has
        self.max_buy = int(player.money//self.item_costs[object])
        if self.item_costs[object]*self.max_buy > player.money:
            self.max_buy -= 1
        while not player_action in ['s','e']:
            #Display shop
            new_line(50)
            print(u'\U0001F9DD')
            print('Merchant')
            print('___________________________________________')
            new_line(1)
            print(''.join(['{}'.format(cell + ' ') for cell in self.shop_inventory])) 
            new_line(1)
            print('___________________________________________')
            #Item description
            item_description(object = object, cost = 'yes')
            print('___________________________________________')
            new_line(1)
            print(''.join(['{}'.format(str(cell) + ' ') if cell in [u'\u2770',self.selection_menu[2]] else cell for cell in self.selection_menu]))
            print('___________________________________________')
            new_line(1)
            print(f'\U0001F4B0: {player.money} | Number in inventory: {player.hidden_item_inventory[object]}')
            new_line(1)
            while not player_action in ['a','d','s','e']:
                print('a(left), d(right), s(select), e(exit): ')
                player_action = msvcrt.getch().decode('utf-8').lower()
            if player_action in ['a','d']:
                new_line(2)
                if player_action == 'a':
                    if self.selection_menu[2] == 1:
                        self.selection_menu[2] = copy.deepcopy(self.max_buy)
                    else:
                        self.selection_menu[2] = self.selection_menu[2] - 1
                else:
                    if self.selection_menu[2] == self.max_buy:
                        self.selection_menu[2] = 1
                    else:
                        self.selection_menu[2] = self.selection_menu[2] + 1
                player_action = ''
            elif player_action == 's':
                player.hidden_item_inventory[object] += self.selection_menu[2]
                if not object in player.observable_item_inventory:
                    free_space = [space for space in range(1,8,2) if player.observable_item_inventory[space] == ''][0]
                    player.observable_item_inventory[free_space] = copy.deepcopy(object)
                player.money -= self.selection_menu[2]*self.item_costs[object]
                self.selection_menu[2] = 1
        return player     


if __name__ == '__main__':
    print("You must run 'python3 start_game.py' to play Castle Descent.")