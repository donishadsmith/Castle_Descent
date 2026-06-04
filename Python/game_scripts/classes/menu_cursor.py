# cursor class
import numpy as np, copy


class menu_cursor_class:
    def __init__(self, position, cursor_movement_dict):
        self.position = position
        self.cursor_movement_dict = cursor_movement_dict

    def move_cursor(self, object, player_action, game_sequence):
        if game_sequence == "inventory":
            menu_type = copy.deepcopy(object.observable_item_inventory)
        elif game_sequence in ["battle", "genie"]:
            menu_type = copy.deepcopy(object.menus[game_sequence])
        else:
            menu_type = copy.deepcopy(object)
        # Get current arrow position
        self.position = np.where(menu_type == "\u2771")[0][0]
        # Clear arrow
        menu_type[self.position] = ""
        # New position
        self.position += self.cursor_movement_dict[player_action]
        # Check length of array for wrapping
        limit = len(menu_type)
        if self.position in [-2, limit]:
            if self.position == -2:
                self.position = limit - 2
            else:
                self.position = 0
        # Add arrow
        menu_type[self.position] = "\u2771"
        if game_sequence == "inventory":
            object.observable_item_inventory = copy.deepcopy(menu_type)
        elif game_sequence in ["battle", "genie"]:
            object.menus[game_sequence] = copy.deepcopy(menu_type)
        else:
            object = copy.deepcopy(menu_type)
        return object


cursor = menu_cursor_class(position=0, cursor_movement_dict={"a": -2, "d": 2})

if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
