import itertools, math, random


class Castle:
    """
    Create randomly generated castle with objects
    """

    def __init__(self):
        self.floor = 0
        # X, Y length of castle
        self.grid = None
        # Max floors/Z
        self.max_floors = random.sample(range(3, 6), 1)[0]
        self.objects = None
        self.merchant = None

    # Columns and rows to be populated
    def products(self):
        return set(
            itertools.product(range(0, self.grid[0], 2), range(0, self.grid[1], 2))
        )

    def spawn(self):
        coordinates = self.products()
        # Merchant
        merchant = random.sample(list(coordinates), 1)
        # Remove coordinate
        coordinates -= set(merchant)
        # Store coord
        self.merchant = merchant[0]
        # Get proportions
        n_cells = len(coordinates)
        # Proportions for objects, remainder will be empty doors
        proportions = {
            "monster": math.floor(n_cells * 0.60),
            "genie": math.floor(n_cells * 0.10),
            "fairy": math.floor(n_cells * 0.10),
        }

        # Populate grid
        self.populate(coordinates, proportions)

    def populate(self, coord, prop):
        for i in prop:
            cells = random.sample(list(coord), prop[i])
            if not self.objects:
                self.objects = {k: i for k in cells}
            else:
                self.objects.update({k: i for k in cells})
            coord -= set(cells)

        # Goal
        goal = random.sample(list(coord), 1)
        self.objects.update({goal[0]: "goal"})
        coord -= set(cells)

        # Remaining coordinates are empty
        self.objects.update({k: None for k in coord})

    # Getting asset behind certain doors
    def get_asset(self, pos):
        if pos in self.doors:
            return self.objects[pos]
        else:
            return None

    @property
    def doors(self):
        return self.objects.keys()

    def empty(self):
        # All possible coordinates
        all_coords = set(
            itertools.product(range(0, self.grid[0]), range(0, self.grid[1]))
        )
        # Remove coordinates that are potentially occupied
        all_coords -= set(self.objects.keys())
        all_coords -= set(tuple(self.merchant))

        return list(all_coords)

    # New X, Y for each floor
    def initialize_grid(self):
        val = random.sample(range(11, 22, 2), 1)[0]
        self.grid = (val, val - 2)

        return self

    def next_floor(self):
        # Increment floor
        self.floor += 1
        # Empty objects
        self.objects = None
        # Create new grid and populate with objects
        self.initialize_grid().spawn()

    def __bool__(self):
        # Determine if castle is in a win state
        return self.floor > self.max_floors

    def __call__(self):
        self.next_floor()

    def __str__(self):
        # Debugging
        state = (
            "Current Castle State\n"
            "====================\n"
            "Current Floor: 0\n"
            f"Max Floors: {self.max_floors}\n"
            f"Grid Initialized: {False if not self.grid else True}\n"
            f"Objects Initialized: {False if not self.grid else True}\n"
            f"Merchant Coordinate: {self.grid}\n"
        )

        return state
