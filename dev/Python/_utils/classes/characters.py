from dataclasses import dataclass

# TODO: Recreate classes
@dataclass
class Player:
    pos = None
    stats = {"HP": 100, "Mana": 20, "Mana_Cost": 5, "Attack": (2, 5), "Money": 0}
    speed = 4

@dataclass
class Zombie:
    pos = None
    speed = 2


#Creating player class to hold player information
import numpy as np, time, copy
#Get functions for adding new lines and displaying the array
from display import *
class player_class:
    def __init__(self, hp, mana, mana_cost, money, hidden_item_inventory, observable_item_inventory, menus, current_coordinate,attack_range, enhanced_attack_range, floor, total_floors,controller,
                 previous_game_update_time,previous_velocity,max_velocity,acceleration, zombie_halt,
                  total_monsters):
        self.hp = hp
        self.mana = mana
        self.mana_cost = mana_cost
        self.money = money
        self.hidden_item_inventory = hidden_item_inventory
        self.observable_item_inventory = observable_item_inventory
        self.current_coordinate = current_coordinate
        self.menus = menus
        self.movement_coordinate = None
        self.encountered_object = None
        self.attack_range = attack_range
        self.enhanced_attack_range = enhanced_attack_range
        self.floor = floor
        self.monster_threshold  = None
        self.previous_velocity = previous_velocity
        self.max_velocity = max_velocity
        self.stimulus_time = None
        self.current_game_update_time = None
        self.previous_game_update_time = previous_game_update_time
        self.acceleration = acceleration
        self.total_floors = total_floors
        self.movement_dimension = None
        self.zombie_halt = zombie_halt
        self.total_monsters = total_monsters
        self.controller = controller
    #Function to calculate object velocity    
    def calculate_player_velocity(self):
        coordinate_difference = tuple(np.array(self.movement_coordinate) - np.array(self.current_coordinate))
        for i,x in enumerate(coordinate_difference):
            if x != 0:
                self.movement_dimension  = i
        coordinate_difference = coordinate_difference[self.movement_dimension]
        coordinate_difference = coordinate_difference//abs(coordinate_difference)
        self.current_velocity = coordinate_difference/(self.current_game_update_time - self.stimulus_time)
        self.acceleration = (self.current_velocity - self.previous_velocity)/(self.current_game_update_time - self.previous_game_update_time)
        self.previous_velocity = copy.deepcopy(self.current_velocity)
        self.previous_game_update_time = copy.deepcopy(self.current_game_update_time)
        if abs(self.current_velocity) > abs(self.max_velocity):
            self.max_velocity = copy.deepcopy(self.current_velocity)
    #Function for player movement
    def movement(self, player_action, castle, castle_info):
        self.movement_coordinate = tuple(np.array(self.current_coordinate) + self.controller[player_action])
        #Wrapping for out of bounds
        for i, x in enumerate(self.movement_coordinate[1:3]):
            #Unlike R, Python supports reverse indexing. However, this game does not support reverse indexing 
            #Because positive coordinates are the keys in the dictionary
            #The length of the dimension can be added to the negative index but I already need to create logic for the out of bound positive index
            if x in [-1,len(castle[0])]:
                dimension = i + 1
                #Convert to numpy array
                self.movement_coordinate = np.array(self.movement_coordinate)
                if self.movement_coordinate[dimension] == -1:
                    self.movement_coordinate[dimension] = len(castle[0]) - 1
                else:
                    self.movement_coordinate[dimension] = self.movement_coordinate[dimension]%len(castle[0])
        #Revert back to tuple
        self.movement_coordinate = tuple(self.movement_coordinate)
        #Get the value from dictionary
        self.encountered_object = copy.deepcopy(castle[self.movement_coordinate])
        #Change the encountered object from the door unicode if it has not been used or defeated 
        if self.encountered_object in [u'\U0001f6aa', u'\u2395']:
            if castle_info[self.movement_coordinate][1] > 0:
                if castle_info[self.movement_coordinate][0] in [u'\U0001f479',u'\U0001F9DB',u'\U0001F409']:
                    self.encountered_object = 'monster'
                elif castle_info[self.movement_coordinate][0] == "D":
                    self.encountered_object = 'next level'
                else:
                    self.encountered_object = castle_info[self.movement_coordinate][0]
    #Function to move to the next level
    def move_to_next_floor_event(self,castle):
        new_line(50)
        display_array(castle = castle, game_sequence = 'next level', hp = self.hp,
        mana = self.mana, floor = self.floor, money = self.money)
        if not self.monster_threshold == 0:
            print('You need to defeat all monsters on this floor to advance.')
        else:
            print(f'You found the stars! You can now advance to floor {self.floor + 2}!')
            castle[self.current_coordinate] = ''
            self.floor += 1
            self.current_coordinate = tuple(np.array(self.current_coordinate) + np.array((1,0,0)))
            castle[self.current_coordinate] = '\U0001F93A' 
            #Calculate new monster threshold
            self.monster_threshold = int(len([floor for floor in self.total_monsters if floor == self.floor]))
        time.sleep(1)
        return castle
    #Function to reset inventory
    def reset_inventory(self):
        current_items = [item for item in self.observable_item_inventory[[1,3,5,7]] if item in self.hidden_item_inventory.keys()]
        current_items = [item for item in current_items if self.hidden_item_inventory[item]]
        self.observable_item_inventory[[1,3,5,7]] = ''
        if len(current_items) > 0:
            #Is numpy array, so support [[]]
            self.observable_item_inventory[list(range(1,len(current_items)*2,2))] = copy.deepcopy(current_items)

import numpy as np
#Creating zombie class to hold player information          
class zombie_class:
    def __init__(self,current_coordinate,controller):
        self.current_coordinate = current_coordinate    
        self.distance_to_player = None
        self.controller = controller
    #Function to calculate chebyshev distance between cooridnates
    def chebyshev_distance(self,a,b):
        distance = max(abs(np.array(a) - np.array(b)))
        return distance
    #Pathfinder
    def pathfinder(self, castle, player, castle_info):
        movement_vector = []
        for key in self.controller:
            possible_coordinate = tuple(np.array(self.current_coordinate) + self.controller[key])
            if not any(num in possible_coordinate for num in [-1,len(castle[0])]):
                if all([possible_coordinate not in castle_info.keys(), castle[possible_coordinate] in ['', '\U0001F93A']]):
                    movement_vector.append(possible_coordinate)
        if player.current_coordinate not in movement_vector:
            #Time = chebyshev distance between the player and zombie divided by max velocity,
            dynamic_t = self.chebyshev_distance(self.current_coordinate,player.current_coordinate)/(player.max_velocity)
            displacement = player.current_velocity*dynamic_t + (player.acceleration*(dynamic_t**2))/2
            check_displacement = float(displacement)
            if check_displacement in [float('nan'),float('inf'),float('-inf')]:
                displacement_vector = np.array((0,0,0))
            else:
                displacement = int(displacement)
                if player.movement_dimension == 1:
                    displacement_vector = np.array((0,displacement,0))
                else:
                    displacement_vector = np.array((0,0,displacement))
            predicted_player_position = tuple(np.array(player.current_coordinate) + displacement_vector)
            distance_to_predicted_player_position = []
            for possible_coordinate in movement_vector:
                distance = self.chebyshev_distance(possible_coordinate,predicted_player_position)
                distance_to_predicted_player_position.append(distance)
            self.current_coordinate = movement_vector[distance_to_predicted_player_position.index(min(distance_to_predicted_player_position))]
        else:
            self.current_coordinate = tuple(player.current_coordinate)
            player.hp = 0
        castle[castle == u'\U0001F9DF']  = ''
        castle[self.current_coordinate] =  u'\U0001F9DF'
        self.distance_to_player = self.chebyshev_distance(self.current_coordinate,player.current_coordinate)
        return castle,player
    #Function so that zombie can move to the next floor
    def move_to_next_floor(self, castle, player):
         available_coordinates =  list(zip(*np.where(castle[player.floor] == '')))   
         max_chebyshev_distance = [self.chebyshev_distance(num,player.current_coordinate[1:2]) for num in available_coordinates]
         castle[castle == u'\U0001F9DF']  = ''
         self.current_coordinate = available_coordinates[max_chebyshev_distance.index(max(max_chebyshev_distance))]
         castle[player.floor][self.current_coordinate] =  u'\U0001F9DF'
         self.current_coordinate = list(zip(*np.where(castle== u'\U0001F9DF')))[0]
         return castle
