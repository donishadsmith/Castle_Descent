import pygame


# Calculate cell size
def cell_size(screen, grid):
    """
    Gets cell size
    """
    cell_width = screen[0] // grid[0]
    cell_height = screen[1] // grid[1]

    return (cell_width, cell_height)


# Render image
def render_grid(screen, element, coord, cell_XY):
    """
    Renders images
    """
    img = pygame.image.load(rf"assets\{element}.png").convert_alpha()
    img = pygame.transform.scale(img, cell_XY)
    x = coord[0] * cell_XY[0]
    y = coord[1] * cell_XY[1]
    screen.blit(img, (x, y))


# Max distance
def max_distance(player, coords):
    """
    Determine max coordinate to spawn zombie at
    """
    coord_dict = {}

    for coord in coords:
        coord_dict.update({coord: chebyshev_distance(player, coord)})

    return max(coord_dict, key=coord_dict.get)


# Chebyshev
def chebyshev_distance(coord_A, coord_B):
    """
    Determine chebyshev distance between two 2D points
    """
    return max(abs(coord_A[0] - coord_B[0]), abs(coord_A[1] - coord_B[1]))
