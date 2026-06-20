# Create local environment
castle_descent <- new.env(parent = globalenv())

# Source files to local environment
files <- dir(recursive = T)[!dir(recursive = T) %in% c("src/env.R", "main.R")]
for(file in files) source(file, local = castle_descent)

# Remove variables from global environment
rm(file)

# Begin game
castle_descent$start_game()
