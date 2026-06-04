# Creating player class to hold player information
import numpy as np, time, copy

# Get functions for adding new lines and displaying the array
from display import *


class player_class:
    def __init__(
        self,
        hp,
        mana,
        mana_cost,
        money,
        hidden_item_inventory,
        observable_item_inventory,
        menus,
        current_coordinate,
        attack_range,
        enhanced_attack_range,
        floor,
        total_floors,
        controller,
        previous_game_update_time,
        previous_velocity,
        max_velocity,
        acceleration,
        zombie_halt,
        total_monsters,
    ):
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
        self.monster_threshold = None
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

    # Function to calculate object velocity
    def calculate_player_velocity(self):
        coordinate_difference = tuple(
            np.array(self.movement_coordinate) - np.array(self.current_coordinate)
        )
        for i, x in enumerate(coordinate_difference):
            if x != 0:
                self.movement_dimension = i
        coordinate_difference = coordinate_difference[self.movement_dimension]
        coordinate_difference = coordinate_difference // abs(coordinate_difference)
        self.current_velocity = coordinate_difference / (
            self.current_game_update_time - self.stimulus_time
        )
        self.acceleration = (self.current_velocity - self.previous_velocity) / (
            self.current_game_update_time - self.previous_game_update_time
        )
        self.previous_velocity = copy.deepcopy(self.current_velocity)
        self.previous_game_update_time = copy.deepcopy(self.current_game_update_time)
        if abs(self.current_velocity) > abs(self.max_velocity):
            self.max_velocity = copy.deepcopy(self.current_velocity)

    # Function for player movement
    def movement(self, player_action, castle, castle_info):
        self.movement_coordinate = tuple(
            np.array(self.current_coordinate) + self.controller[player_action]
        )
        # Wrapping for out of bounds
        for i, x in enumerate(self.movement_coordinate[1:3]):
            # Unlike R, Python supports reverse indexing. However, this game does not support reverse indexing
            # Because positive coordinates are the keys in the dictionary
            # The length of the dimension can be added to the negative index but I already need to create logic for the out of bound positive index
            if x in [-1, len(castle[0])]:
                dimension = i + 1
                # Convert to numpy array
                self.movement_coordinate = np.array(self.movement_coordinate)
                if self.movement_coordinate[dimension] == -1:
                    self.movement_coordinate[dimension] = len(castle[0]) - 1
                else:
                    self.movement_coordinate[dimension] = self.movement_coordinate[
                        dimension
                    ] % len(castle[0])
        # Revert back to tuple
        self.movement_coordinate = tuple(self.movement_coordinate)
        # Get the value from dictionary
        self.encountered_object = copy.deepcopy(castle[self.movement_coordinate])
        # Change the encountered object from the door unicode if it has not been used or defeated
        if self.encountered_object in ["\U0001f6aa", "\u2395"]:
            if castle_info[self.movement_coordinate][1] > 0:
                if castle_info[self.movement_coordinate][0] in [
                    "\U0001f479",
                    "\U0001f9db",
                    "\U0001f409",
                ]:
                    self.encountered_object = "monster"
                elif castle_info[self.movement_coordinate][0] == "D":
                    self.encountered_object = "next level"
                else:
                    self.encountered_object = castle_info[self.movement_coordinate][0]

    # Function to move to the next level
    def move_to_next_floor_event(self, castle):
        new_line(50)
        display_array(
            castle=castle,
            game_sequence="next level",
            hp=self.hp,
            mana=self.mana,
            floor=self.floor,
            money=self.money,
        )
        if not self.monster_threshold == 0:
            print("You need to defeat all monsters on this floor to advance.")
        else:
            print(
                f"You found the stars! You can now advance to floor {self.floor + 2}!"
            )
            castle[self.current_coordinate] = ""
            self.floor += 1
            self.current_coordinate = tuple(
                np.array(self.current_coordinate) + np.array((1, 0, 0))
            )
            castle[self.current_coordinate] = "\U0001f93a"
            # Calculate new monster threshold
            self.monster_threshold = int(
                len([floor for floor in self.total_monsters if floor == self.floor])
            )
        time.sleep(1)
        return castle

    # Function to reset inventory
    def reset_inventory(self):
        current_items = [
            item
            for item in self.observable_item_inventory[[1, 3, 5, 7]]
            if item in self.hidden_item_inventory.keys()
        ]
        current_items = [
            item for item in current_items if self.hidden_item_inventory[item]
        ]
        self.observable_item_inventory[[1, 3, 5, 7]] = ""
        if len(current_items) > 0:
            # Is numpy array, so support [[]]
            self.observable_item_inventory[
                list(range(1, len(current_items) * 2, 2))
            ] = copy.deepcopy(current_items)


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
