Castle <- setRefClass(
  "Class to generate and store information about Castle",
  fields = list(
    # Tracker for current castle floor
    curr_floor = "numeric",
    # Castle grid of current floor
    grid = "array",
    # Numeric representing x, y, and z of Castle
    dims = "numeric",
    # Track location of objects in castle
    df = "data.frame"
  ),
  methods = list(
    generate_df = function() {
      df <<- data.frame("X" = NA, "Y" = NA, "Object" = NA, "HP" = NA, "Money" = NA)
    },
    get_dims = function() {
      # Area
      x <- y <- sample(seq(9, 13, 2), 1)
      # Depth
      if (isTRUE(curr_floor == 1)) {
        z <- sample(3:6, 1)
        dims <<- c(x, y, z)
      } else {
        dims <<- c(x, y, dims[3])
      }
    },
    generate_grid = function() {
      # Initiate empty array for castle
      grid <<- array("", dim = c(dims[1], dims[2]))
      # Evenly spaced objects
      x <- y <- seq(2, (dims[1] - 1), 2)
      # Spawn doors
      grid[x, y] <<- .UNICODE$door
    },
    populate_df = function() {
      # Cells to populate
      cells <- which(grid == .UNICODE$door, arr.ind = TRUE)
      n_cells <- nrow(cells)

      for (object in names(.SPAWN_RATES)) {
        N <- if (object %in% c("exit", "merchant")) .SPAWN_RATES[object] else floor(n_cells * .SPAWN_RATES[object])
        pos <- sample(seq(nrow(cells)), N, replace = FALSE)
        rows <- if (all(is.na(df))) 1:(nrow(df) + length(pos) - 1) else (nrow(df) + 1):(nrow(df) + length(pos))
        df[rows, 1:2] <<- cells[pos, ]

        if (object == "monster") {
          # Unicode
          df[rows, 3] <<- sample(.UNICODE$events[["monster"]], length(rows), replace = TRUE)
          # HP for monsters; HP increases by a factor of 10 for each floor except first floor
          adjust <- if (curr_floor == 1) 1 else curr_floor * 10
          df[rows, 4] <<- sample(10:20 + adjust, length(rows), replace = TRUE)
          # Money for defeating monsters; half the original hp
          df[rows, 5] <<- df[rows, 4] * 0.5
        } else if (object == "exit") {
          # Unicode
          unicode_pos <- ifelse(isFALSE(curr_floor == dims[3]), 1, 2)
          df[rows, 3] <<- sample(.UNICODE$events[["exit"]][unicode_pos], length(rows), replace = TRUE)
        } else {
          # Unicode
          df[rows, 3] <<- sample(.UNICODE$events[[object]], length(rows), replace = TRUE)
        }

        # Remove used coordinates
        cells <- cells[-pos, ]
      }
    },
    initialize = function(add_level = FALSE) {
      curr_floor <<- if (add_level) curr_floor + 1 else 1
      # Create base df
      generate_df()
      # Get dimensions
      get_dims()
      # Create grid
      generate_grid()
      # Populate df
      populate_df()
    }
  )
)

# Check if coordinate is in dataframe and return a boolean
.check_coord <- function(input, df) {
  return(any(sapply(seq(nrow(df)), function(row) identical(input, df[row, 1:3]))))
}

