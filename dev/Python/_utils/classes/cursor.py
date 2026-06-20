# Cursor for navigating menus
import numpy as np, copy

class Cursor:
    """
    Cursor for controlling different menus
    """
    def __init__(self):
        self.position = 0
        self.cursor_movement_dict = {"a": -2, "d": 2}

    # TODO: Refactor remaining functions
    def get_menu(self, object, sequence):
        if sequence == "inventory":
            menu_type = copy.deepcopy(object.observable_item_inventory)
        elif sequence in ["battle", "genie"]:
            menu_type = copy.deepcopy(object.menus[sequence])
        else:
            menu_type = copy.deepcopy(object)
        
    def move_cursor(self, object, player_action, sequence):
        if sequence == "inventory":
            menu_type = copy.deepcopy(object.observable_item_inventory)
        elif sequence in ["battle", "genie"]:
            menu_type = copy.deepcopy(object.menus[sequence])
        else:
            menu_type = copy.deepcopy(object)
        #Get current arrow position
        self.position = np.where(menu_type == u"\u2771")[0][0]
        #Clear arrow
        menu_type[self.position] = ""
        #New position
        self.position += self.cursor_movement_dict[player_action]
        #Check length of array for wrapping
        limit = len(menu_type)
        if self.position in [-2, limit]:
            if self.position == -2:
                self.position = limit - 2
            else:
                self.position = 0
        # Add arrow
        menu_type[self.position] = u"\u2771"
        if sequence == "inventory":
            object.observable_item_inventory = copy.deepcopy(menu_type)
        elif sequence in ["battle","genie"]:
            object.menus[sequence] = copy.deepcopy(menu_type)
        else:
            object = copy.deepcopy(menu_type)       
        return object
