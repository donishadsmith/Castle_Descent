import random, time, msvcrt, numpy as np
from display import *
from inventory import *
from menu_cursor import cursor


def genie_event(castle, player, castle_info):
    castle[player.movement_coordinate] = player.encountered_object
    display_array(castle=castle, game_sequence="non-battle", player=player)
    print("You encountered a genie!")
    player_choice = ""
    player_action = ""
    cursor.position = 0
    if "Reduce Mana Cost" in player.menus["genie"]:
        while not player_action == "s":
            cursor.position = np.where(player.menus["battle"] == "\u2771")[0][0]
            display_array(castle=castle, game_sequence="non-battle", player=player)
            new_line(1)
            print("___________________________________________")
            new_line(1)
            print("".join(["{}".format(cell + " ") for cell in player.menus["genie"]]))
            new_line(1)
            print("___________________________________________")
            new_line(1)
            while not player_action in ["a", "d", "s"]:
                print("a(left), d(right), s(select): ")
                player_action = msvcrt.getch().decode("utf-8").lower()
            if player_action in ["a", "d"]:
                player = cursor.move_cursor(
                    object=player, player_action=player_action, game_sequence="genie"
                )
                player_action = ""
            else:
                player_choice = player.menus["genie"][cursor.position + 1]
    else:
        player_choice = "Increase Attack"
    display_array(castle=castle, game_sequence="non-battle", player=player)
    if player_choice == "Increase Attack":
        attack_increase = random.sample([num for num in range(1, 6)], 1)[0]
        print(
            f"Your Attack range and Enhanced Attack range increased by {attack_increase} points."
        )
        new_line(1)
        player.attack_range = list(
            map(lambda a: a + attack_increase, player.attack_range)
        )
        player.enhanced_attack_range = list(
            map(lambda a: a + attack_increase, player.enhanced_attack_range)
        )
        print(
            f"New Attack range: {min(player.attack_range)}:{max(player.attack_range)}"
        )
        new_line(1)
        print(
            f"New Enhanced Attack range: {min(player.enhanced_attack_range)}:{max(player.enhanced_attack_range)}"
        )
    elif player_choice == "Reduce Mana Cost":
        decrease_mana_cost = random.sample([num for num in range(1, 6)], 1)[0]
        player.mana_cost -= decrease_mana_cost
        if player.mana_cost < 1:
            player.mana_cost = 1
            player.menus["genie"] = player.menus["genie"][0:2]
        player.menus["battle"][3] = f"Enhanced Attack({player.mana_cost} mana)"
        print(f"Your Enhanced Attack now costs {player.mana_cost} mana.")

    # Changing dictionary number to zero to prevent this event from activating again
    castle_info[player.movement_coordinate][1] = 0
    time.sleep(1)
    # Adding back door
    castle[player.movement_coordinate] = "\U0001f6aa"
    player.menus["genie"][cursor.position] = ""
    player.menus["genie"][0] = "\u2771"
    time.sleep(1)
    return castle, player, castle_info


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
