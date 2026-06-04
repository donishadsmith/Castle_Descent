import random, numpy as np


def castle_create():
    # x,y of castle
    castle_area = random.sample([num for num in range(9, 17, 2)], 1)[0]
    # z of castle
    castle_z_length = random.sample([num for num in range(3, 7)], 1)[0]
    # dtype = 'U1' so that numpy array can store unicode
    castle = np.array(
        np.zeros(shape=[castle_z_length, castle_area, castle_area]), dtype="object"
    )
    # From an array containing zeroes to an array of block unicode
    castle[:] = ""
    # Designating specfic areas for monster, genie, fairy, stairs, etc objects
    object_spawn_locations = [num for num in range(1, castle_area - 1, 2)]
    for num in object_spawn_locations:
        castle[:, num, object_spawn_locations] = "1"
    # Spawn player by randomly sampling x,y coordinate from list
    free_space = random.sample(list(zip(*np.where(castle[0] == ""))), 1)[0]
    castle[0][free_space] = "\U0001f93a"
    # Spawn zombie at max euclidean distance
    # Get available coordinates
    player_coord = list(zip(*np.where(castle[0] == "\U0001f93a")))[0]
    available_coordinates = list(zip(*np.where(castle[0] == "")))
    max_chebyshev_distance = [
        max(abs(np.array(num) - np.array(player_coord)))
        for num in available_coordinates
    ]
    castle[0][
        available_coordinates[max_chebyshev_distance.index(max(max_chebyshev_distance))]
    ] = "\U0001f9df"
    # Spawn objects
    # Spawn stairs and exit
    object_info = []
    for floor in range(0, (castle_z_length - 1)):
        spawnable_coord = random.sample(list(zip(*np.where(castle[floor] == "1"))), 1)[
            0
        ]
        castle[floor][spawnable_coord] = "D"
        castle[floor + 1][spawnable_coord] = "A"
        object_info.append([floor, spawnable_coord[0], spawnable_coord[1], "D", 1])
        object_info.append([floor + 1, spawnable_coord[0], spawnable_coord[1], "A", 1])
        if floor == castle_z_length - 2:
            spawnable_coord = random.sample(
                list(zip(*np.where(castle[floor + 1] == "1"))), 1
            )[0]
            castle[floor + 1][spawnable_coord] = "\u2395"
            object_info.append(
                [floor + 1, spawnable_coord[0], spawnable_coord[1], "\u2395", 1]
            )
    # Spawn fairies and genies
    for floor in range(0, castle_z_length):
        # Spawn fairies
        fairy_coordinates = random.sample(
            list(zip(*np.where(castle[floor] == "1"))), castle_area // 5
        )
        for fairy_coordinate in fairy_coordinates:
            castle[floor][fairy_coordinate] = "\U0001f9da"
            object_info.append(
                [floor, fairy_coordinate[0], fairy_coordinate[1], "\U0001f9da", 1]
            )
        # Spawn genies
        genie_coordinates = random.sample(
            list(zip(*np.where(castle[floor] == "1"))), castle_area // 5
        )
        for genie_coordinate in genie_coordinates:
            castle[floor][genie_coordinate] = "\U0001f9de"
            object_info.append(
                [floor, genie_coordinate[0], genie_coordinate[1], "\U0001f9de", 1]
            )
        # Spawn merchant
        merchant_coordinate = random.sample(
            list(zip(*np.where(castle[floor] == "1"))), 1
        )[0]
        castle[floor][merchant_coordinate] = "\U0001f9dd"
    # Spawn monsters
    base_hp_vector = [num for num in range(10, 21)]
    for floor in range(0, castle_z_length):
        monster_coordinates = list(zip(*np.where(castle[floor] == "1")))
        for monster_coordinate in monster_coordinates:
            monster_object = random.sample(
                ["\U0001f479", "\U0001f9db", "\U0001f409"], 1
            )[0]
            castle[floor][monster_coordinate] = monster_object
            hp = random.sample(base_hp_vector, 1)[0]
            object_info.append(
                [
                    floor,
                    monster_coordinate[0],
                    monster_coordinate[1],
                    monster_object,
                    hp,
                    hp * 0.5,
                ]
            )
        base_hp_vector = [num + 20 for num in base_hp_vector]
    # Put information in dictionary
    # Coordinates will be the keys and the unicode and and number will be the values
    # Values in a list to ensure that these values can be changes when they need to be
    castle_info = {}
    for obj in object_info:
        if not obj[3] in ["\U0001f479", "\U0001f9db", "\U0001f409"]:
            castle_info[(obj[0], obj[1], obj[2])] = [obj[3], obj[4]]
        else:
            castle_info[(obj[0], obj[1], obj[2])] = [obj[3], obj[4], obj[5]]
    # Spawn doors
    for key in castle_info:
        castle[key] = "\U0001f6aa"
    return castle, castle_info


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
