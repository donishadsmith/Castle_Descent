import numpy as np


def generate_objects(
    player_class,
    zombie_class,
    merchant_class,
    castle,
    castle_info,
    player_controller,
    zombie_movement,
):
    # Add values to player and zombie attributes
    player = player_class(
        hp=100,
        mana=100,
        money=0,
        hidden_item_inventory=dict(
            {
                "\U0001f52e": 0,
                "\U0001f371": 0,
                "\U0001f50e": 0,
                "\U0001f9ea": 0,
            }
        ),
        mana_cost=20,
        observable_item_inventory=np.array(np.zeros(shape=8), dtype="U1"),
        menus={
            "battle": np.array(np.zeros(shape=8), dtype="object"),
            "genie": np.array(np.zeros(shape=4), dtype="object"),
        },
        zombie_halt=0,
        attack_range=[num for num in range(5, 11)],
        enhanced_attack_range=[num for num in range(20, 25)],
        floor=0,
        controller=player_controller,
        current_coordinate=list(zip(*np.where(castle == "\U0001f93a")))[0],
        total_floors=len(castle),
        # For velocity and acceleration, there is code in Classes.py
        # to get the correct dimension
        max_velocity=0,
        previous_velocity=0,
        previous_game_update_time=0,
        acceleration=0,
        total_monsters=[
            floor[0]
            for floor in [
                (key[0], object[0])
                for key, object in castle_info.items()
                if object[0] in ["\U0001f479", "\U0001f9db", "\U0001f409"]
            ]
        ],
    )
    # Add cursor to observable inventory and menus
    player.observable_item_inventory[:] = ""
    player.observable_item_inventory[0] = "\u2771"
    for menu in player.menus.keys():
        player.menus[menu][:] = ""
        player.menus[menu][0] = "\u2771"
    # Add words to player menus
    player.menus["battle"][list(range(1, len(player.menus["battle"]), 2))] = [
        "Attack",
        f"Enchanced Attack({player.mana_cost} mana)",
        "Inventory",
        "Run",
    ]
    player.menus["genie"][list(range(1, len(player.menus["genie"]), 2))] = [
        "Increase Attack",
        "Reduce Mana Cost",
    ]
    # Calculate the number of monsters needed to be defeated
    player.monster_threshold = int(
        len([floor for floor in player.total_monsters if floor == player.floor]) * 0.60
    )
    # Get zombie coordinate
    zombie = zombie_class(
        current_coordinate=list(zip(*np.where(castle == "\U0001f9df")))[0],
        controller=zombie_movement,
    )
    # Calculate initial distance between zombie and player
    zombie.distance_to_player = zombie.chebyshev_distance(
        zombie.current_coordinate, player.current_coordinate
    )
    # Create merchant
    merchant = merchant_class(
        item_costs={
            "\U0001f52e": 50,
            "\U0001f371": 10,
            "\U0001f50e": 500,
            "\U0001f9ea": 40,
        },
        shop_inventory=np.array(np.zeros(shape=8), dtype="object"),
        selection_menu=np.array(np.zeros(shape=4), dtype="object"),
    )
    merchant.shop_inventory[:] = ""
    merchant.shop_inventory[list(range(1, 8, 2))] = [
        "\U0001f52e",
        "\U0001f371",
        "\U0001f50e",
        "\U0001f9ea",
    ]
    merchant.shop_inventory[0] = "\u2771"
    merchant.selection_menu[:] = ["Buy: ", "\u2770", 1, "\u2771"]
    return player, zombie, merchant
