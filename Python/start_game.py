import msvcrt, sys, os, time, os.path as op, numpy as np, glob

if "win" not in sys.platform:
    raise SystemError("Castle Descent only runs on Windows platforms.")
# Get directory of file
work_dir = os.path.dirname(os.path.abspath(__file__))
# Add directories to system path
sys.path.append(op.join(work_dir, "game_scripts"))
sys.path.append(op.join(work_dir, "game_scripts", "classes"))
sys.path.append(op.join(work_dir, "game_scripts", "events"))

# Get names of files
module_names = [
    op.basename(file).split(".")[0]
    for dir_path in [
        op.join(work_dir, "game_scripts"),
        op.join(work_dir, "game_scripts", "events"),
        op.join(work_dir, "game_scripts", "classes"),
    ]
    for file in glob.glob(op.join(dir_path, "*.py"))
]

# Put callable objects from modules in globals
for module_name in module_names:
    module = __import__(module_name)
    functions = [
        object
        for object in dir(module)
        if callable(getattr(module, object))
        if object not in globals()
    ]
    globals().update({name: getattr(module, name) for name in functions})

from controllers import *

# Used to determine the intro the player recieves depending on how many times the game is repeated
iteration = 0


# While loop of actual game
def start_game():
    # Get outputs from castle_create() function
    castle, castle_info = castle_create()
    # Generate player, zombie and merchant
    player, zombie, merchant = generate_objects(
        player_class=player_class,
        zombie_class=zombie_class,
        merchant_class=merchant_class,
        castle=castle,
        castle_info=castle_info,
        player_controller=player_controller,
        zombie_movement=zombie_movement,
    )
    # Introduction changes depending on whether or not this is the first iteration of the session
    if iteration == 0:
        print("Welcome to Castle Descent!")
        new_line(1)
        time.sleep(1)
        print(
            "The objective of the game is to descend the bottom of the castle while avoiding the zombie."
        )
        new_line(1)
        time.sleep(1)
        print("You will be starting at the top of the castle!")
        time.sleep(1)
    else:
        print("New game.")
        time.sleep(1)
    # While loop of the actual game
    # The loop is program if the player is eaten by the zombie, has their health droppped to 0, or finds the exit
    game_status = "play"
    while not any(
        [
            player.hp == 0,
            zombie.distance_to_player == 0,
            player.encountered_object == "\u2395",
            game_status == "quit",
        ]
    ):
        # Print game screen and corresponding prompts
        object = ""
        # Door revealed if player defeats certain number of monsters, this adds an incentive to defeat monsters
        # instead of running
        if player.monster_threshold == 0:
            for key, object in castle_info.items():
                if key[0] == player.floor and object[0] in ["D", "\u2395"]:
                    coord = key
                    object = object[0]
            castle[coord] = "\u2395"
        # Displays current floor/grid

        display_array(castle=castle, game_sequence="non-battle", player=player)
        match object:
            case "D":
                print("The location of the stairs has been revealed!")
            case "\u2395":
                print("The location of the exit has been revealed!")
            case _:
                print(
                    f"Defeat all monsters on this floor to advance: {player.monster_threshold} left."
                )
        # Information if player finds a crystal ball and uses the crystal ball in their inventory
        # To stop the zombie for 10 - 20 steps
        if player.zombie_halt > 0:
            new_line(1)
            print(f"Number of steps before zombie can move: {player.zombie_halt} left.")
        new_line(1)
        # Player input
        player_action = ""
        # Get approximate time of stimulus presention. The stimulus in the prompt
        player.stimulus_time = time.time()
        # while not player_action in player_controller:
        while not player_action in player_controller:
            print("w(up), a(left), s(down), d(right), inventory(i), quit(q): ")
            player_action = msvcrt.getch().decode("utf-8").lower()
        # Events are determined by encountered objects
        if player_action == "i":
            player.encountered_object = player.controller["i"]
        # Else, the movement coordinate is updated and the dictionary is accessed, if the movement coordinate corresponds to the door
        elif player_action == "q":
            game_status = player.controller["q"]
            player.encountered_object = "quit"
        else:
            # Get new movement coordinate and encountered object
            player.movement(
                player_action=player_action, castle=castle, castle_info=castle_info
            )
        # Event for movable spaces, empty doors, and the zombie
        if player.encountered_object in ["", "\U0001f6aa", "\U0001f9df"]:
            # If the current coordinate is the player, it is erased
            # Prevents door from being erased if player is hiding behind it
            if castle[player.current_coordinate] == "\U0001f93a":
                castle[player.current_coordinate] = ""
            if player.encountered_object in ["", "\U0001f6aa"]:
                # Update with new player unicode if new space is empty
                if player.encountered_object == "":
                    castle[player.movement_coordinate] = "\U0001f93a"
                # Get velocity and acceleration
                # Velocity and acceleration depends on player reaction time plus the it takes for the coordinate to update
                # This creates more interesting movement since reaction time is more variable than update time for the code

                # Small delay to prevent a divide by 0 error
                time.sleep(0.01)
                player.current_game_update_time = time.time()
                player.calculate_player_velocity()
                player.current_coordinate = tuple(player.movement_coordinate)
                # Update player coordinate
                player.current_coordinate = tuple(player.movement_coordinate)
                if player.zombie_halt == 0:
                    # Zombie pathfinder finds the shortest path to the player's predicted position
                    # Results in coordinates that are out of bounds but this is what causes the interesting movement
                    # Pathfinder locates the possible coordinate that is the shortest chebyshev distance to the predicted coordinate
                    # Zombie is only allowed to move into empty spaces or spaces containing the player
                    castle, player = zombie.pathfinder(
                        castle=castle, player=player, castle_info=castle_info
                    )
                else:
                    # If the zombie is halted, every iteration reduces the number of steps by 1
                    player.zombie_halt -= 1
            # Events if player encounters empty door
            elif player.encountered_object == "\U0001f9df":
                player.current_coordinate = tuple(player.movement_coordinate)
                zombie.distance_to_player = 0
        else:
            # If the encountered item is not
            # Match case to relevent event function depending on encountered items
            match player.encountered_object:
                case "\U0001f9da":
                    castle, player, castle_info = fairy_event(
                        castle=castle, player=player, castle_info=castle_info
                    )
                case "\U0001f9de":
                    castle, player, castle_info = genie_event(
                        castle=castle, player=player, castle_info=castle_info
                    )
                case "monster":
                    # Get monster unicode
                    player.encountered_object = castle_info[player.movement_coordinate][
                        0
                    ]
                    castle, player, castle_info = monster_event(
                        castle=castle, player=player, castle_info=castle_info
                    )
                case "next level":
                    castle = player.move_to_next_floor_event(castle=castle)
                    if not zombie.current_coordinate[0] == player.current_coordinate[0]:
                        castle = zombie.move_to_next_floor(castle=castle, player=player)
                case "A":
                    upstairs_event(castle, player)
                case "inventory":
                    player = inventory(player=player, game_sequence="free movement")
                case "\U0001f9dd":
                    player = merchant.menu(player=player, game_sequence="merchant")
    # Events if loop is broken
    if zombie.distance_to_player == 0:
        player.hp = 0
        prompt = "You were eaten by the zombie"
    elif player.encountered_object == "\u2395":
        prompt = "You found the exit!"
        castle[player.movement_coordinate] = "\u2395"
    else:
        new_line(2)
        print("Thank you for playing Castle Descent!")
        sys.exit()
    display_array(castle=castle, game_sequence="non-battle", player=player)
    print(prompt)
    # Retry
    player_action = ""
    new_line(1)
    while not player_action in ["y", "n"]:
        print("Would you like to play a new game? Yes (y) or no (n): ")
        player_action = msvcrt.getch().decode("utf-8").lower()
    if player_action in ["yes", "y"]:
        start_game()
    else:
        new_line(1)
        print("Thank you for playing Castle Descent!")


if __name__ == "__main__":
    start_game()
