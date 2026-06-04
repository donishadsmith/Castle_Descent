import numpy as np


# Creating zombie class to hold player information
class zombie_class:
    def __init__(self, current_coordinate, controller):
        self.current_coordinate = current_coordinate
        self.distance_to_player = None
        self.controller = controller

    # Function to calculate chebyshev distance between cooridnates
    def chebyshev_distance(self, a, b):
        distance = max(abs(np.array(a) - np.array(b)))
        return distance

    # Pathfinder
    def pathfinder(self, castle, player, castle_info):
        movement_vector = []
        for key in self.controller:
            possible_coordinate = tuple(
                np.array(self.current_coordinate) + self.controller[key]
            )
            if not any(num in possible_coordinate for num in [-1, len(castle[0])]):
                if all(
                    [
                        possible_coordinate not in castle_info.keys(),
                        castle[possible_coordinate] in ["", "\U0001f93a"],
                    ]
                ):
                    movement_vector.append(possible_coordinate)
        if player.current_coordinate not in movement_vector:
            # Time = chebyshev distance between the player and zombie divided by max velocity,
            dynamic_t = self.chebyshev_distance(
                self.current_coordinate, player.current_coordinate
            ) / (player.max_velocity)
            displacement = (
                player.current_velocity * dynamic_t
                + (player.acceleration * (dynamic_t**2)) / 2
            )
            check_displacement = float(displacement)
            if check_displacement in [float("nan"), float("inf"), float("-inf")]:
                displacement_vector = np.array((0, 0, 0))
            else:
                displacement = int(displacement)
                if player.movement_dimension == 1:
                    displacement_vector = np.array((0, displacement, 0))
                else:
                    displacement_vector = np.array((0, 0, displacement))
            predicted_player_position = tuple(
                np.array(player.current_coordinate) + displacement_vector
            )
            distance_to_predicted_player_position = []
            for possible_coordinate in movement_vector:
                distance = self.chebyshev_distance(
                    possible_coordinate, predicted_player_position
                )
                distance_to_predicted_player_position.append(distance)
            self.current_coordinate = movement_vector[
                distance_to_predicted_player_position.index(
                    min(distance_to_predicted_player_position)
                )
            ]
        else:
            self.current_coordinate = tuple(player.current_coordinate)
            player.hp = 0
        castle[castle == "\U0001f9df"] = ""
        castle[self.current_coordinate] = "\U0001f9df"
        self.distance_to_player = self.chebyshev_distance(
            self.current_coordinate, player.current_coordinate
        )
        return castle, player

    # Function so that zombie can move to the next floor
    def move_to_next_floor(self, castle, player):
        available_coordinates = list(zip(*np.where(castle[player.floor] == "")))
        max_chebyshev_distance = [
            self.chebyshev_distance(num, player.current_coordinate[1:2])
            for num in available_coordinates
        ]
        castle[castle == "\U0001f9df"] = ""
        self.current_coordinate = available_coordinates[
            max_chebyshev_distance.index(max(max_chebyshev_distance))
        ]
        castle[player.floor][self.current_coordinate] = "\U0001f9df"
        self.current_coordinate = list(zip(*np.where(castle == "\U0001f9df")))[0]
        return castle


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
