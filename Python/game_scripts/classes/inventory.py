import copy, random, time, numpy as np, msvcrt
from display import *
from item_description import item_description
from menu_cursor import cursor


def inventory(player, game_sequence):
    cursor.position = 0
    player_action = ""
    object = copy.deepcopy(player.observable_item_inventory[1])
    while not player_action == "e":
        new_line(50)
        print("Inventory")
        print("___________________________________________")
        new_line(1)
        print(
            "".join(
                ["{}".format(cell + " ") for cell in player.observable_item_inventory]
            )
        )
        new_line(1)
        print("___________________________________________")
        # Item description
        item_description(object=object, cost="no")
        print("___________________________________________")
        new_line(1)
        if not object == "":
            new_line(1)
            print(f"Number in inventory: {player.hidden_item_inventory[object]}")
            new_line(1)
        while not player_action in ["a", "d", "s", "e"]:
            print("a(left), d(right), s(select), e(exit): ")
            player_action = msvcrt.getch().decode("utf-8").lower()
        if player_action in ["a", "d"]:
            player = cursor.move_cursor(
                object=player, player_action=player_action, game_sequence="inventory"
            )
            player_action = ""
        if player_action == "s":
            if any(
                [
                    game_sequence == "free movement",
                    game_sequence == "battle"
                    and object in ["\U0001f371", "\U0001f9ea"],
                ]
            ):
                player = use_item(player=player, object=object)
                if 0 in player.hidden_item_inventory.values():
                    player.reset_inventory()
                player_action = ""
            elif object != "":
                new_line(50)
                print("Inventory")
                print("___________________________________________")
                new_line(1)
                print(
                    "".join(
                        [
                            "{}".format(cell + " ")
                            for cell in player.observable_item_inventory
                        ]
                    )
                )
                new_line(1)
                print("___________________________________________")
                new_line(1)
                print("Cannot use this item during battle.")
                player_action = ""
                time.sleep(1)
            else:
                player_action = ""

        # Get object arrow is pointing to
        object = copy.deepcopy(player.observable_item_inventory[cursor.position + 1])
    # If loop is broken, reset cursor position
    # Erase cursor
    player.observable_item_inventory[
        np.where(player.observable_item_inventory == "\u2771")[0][0]
    ] = ""
    player.observable_item_inventory[0] = "\u2771"
    return player


def use_item(player, object):
    new_line(50)
    # Reduce number in inventory
    player.hidden_item_inventory[object] -= 1
    print("Inventory")
    print("___________________________________________")
    new_line(1)
    print(
        "".join(["{}".format(cell + " ") for cell in player.observable_item_inventory])
    )
    new_line(1)
    print("___________________________________________")
    new_line(1)
    match object:
        case "\U0001f52e":
            player.zombie_halt = random.sample([num for num in range(10, 21)], 1)[0]
            print(f"Zombie halted for {player.zombie_halt} steps")
            print("Temporarily halts zombie movement.")
        case "\U0001f371":
            if player.hp == 100:
                print("You are already at full health.")
                # Add back item
                player.hidden_item_inventory[object] += 1
            elif player.hp + 20 > 100:
                restore = player.hp + 20 + (100 - (player.hp + 20))
                print(f"Your HP increased by {abs(100 - player.hp)} points.")
                player.hp = copy.deepcopy(restore)
            else:
                print("Your HP increased by 20 points.")
                player.hp += 20
        case "\U0001f50e":
            player.monster_threshold = 0
            print("The door has been revealed.")
        case "\U0001f9ea":
            if player.mana == 100:
                print("Your mana is already maxed out.")
                # Add back item
                player.hidden_item_inventory[object] += 1
            elif player.mana + 30 > 100:
                restore = player.mana + 30 + (100 - (player.mana + 30))
                print(f"Your mana increased by {abs(100 - player.mana)} points.")
                player.mana = copy.deepcopy(restore)
            else:
                print("Your mana increased by 30 points.")
                player.mana += 30

    time.sleep(1)
    return player


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
