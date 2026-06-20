# TODO: Refactor
#Variable used to determine if the player is greeted with the 
#welcome screen or not
iteration <- 0

start_game <- function(){
  
  #Using castle-create to generate a new random game board
  castle_data <- castle_create()
  #creating a class for the player
  player <- player_class(current_coordinate = which(castle_data$castle=="\U1F93A", arr.ind = T),
                        hidden_item_inventory = list("\U1F52E" = 0, "\U1F371" = 0, "\U0001f50e" = 0,"\U1F9EA" = 0),
                        observable_item_inventory = matrix(c("\U2771",rep("",7)), ncol = 8),
                        mana_cost = 20,
                        zombie_halt = 0,
                        hp = 100,
                        mana = 100,
                        money = 0,
                        attack_range = 5:10,
                        enhanced_attack_range = 20:25,
                        #wasd controls will be added to the players current 3d coordinate to 
                        #obtain a new coordinate corresponding to the direction
                        movement_dict = list("w" = c(-1,0,0), "a" = c(0,-1,0), "s" = c(1,0,0),  "d" = c(0,1,0)),
                        floor = 1, 
                        total_floors = length(castle_data$castle[1,1,]),
                        encountered_object = "",
                        max_velocity = 0,
                        acceleration = 0,
                        previous_velocity = 0,
                        previous_game_update_time = 0,
                        monster_threshold = round(length(which(castle_data$dataframe$z == 1 & castle_data$dataframe$object %in% c("\U1F479","\U1F9DB","\U1F409"))),0))
  
  #Add player menus
  player$menus <- list("genie" = matrix(c("\U2771","Increase Attack","","Reduce Mana Cost"), ncol = 4)
                       ,"battle" = matrix(c("\U2771","Attack","","Enhanced Attack(20 mana)","",
                                            "Inventory","","Run"),ncol = 8))
  zombie <- zombie_class(current_coordinate = which(castle_data$castle=="\U1F9DF", arr.ind = T),
                        movement_dict = list("w" = c(-1,0,0),"a" = c(0,-1,0),"s" = c(1,0,0),"d" = c(0,1,0),
                                             "diag_up_left" = c(-1,-1,0),"diag_up_right" = c(-1,1,0),
                                             "diag_down_left" = c(1,-1,0),"diag_down_right" = c(1,1,0)),
                        floor = 1)

  zombie$distance_to_player <- zombie$chebyshev_distance(zombie$current_coordinate,player$current_coordinate)
  #######################################Setup Complete########################################
  #if the player decides to play a new game, 
  #they do not need to be greeted with the welcome screen and objective again
  
  if(iteration == 0){
    cat("Welcome to Castle Descent!")
    new_line(2)
    Sys.sleep(1)
    cat("The objective of the game is to descend the bottom of the castle while avoiding the zombie.")
    new_line(2)
    Sys.sleep(1)
    cat("You will be starting at the top of the castle!")
    new_line(2)
    Sys.sleep(1)
  }else{
    new_line(50)
    cat("New game.")
    new_line(1)
    Sys.sleep(1)
  }
  game_status <- "play"
  while(!(player$encountered_object=="\U2395"  | player$hp <= 0 | zombie$distance_to_player == 0| game_status == "quit")){
    #If player kills a certain number of monsters, the stairs or exit is revealed
    object <- ""
    if(player$monster_threshold == 0){
      #Add multiple spaces so that only a single cat out of the game board is on the screen 
      #Obtain coordinates for downstairs or the exit depending on player"s current floor
      get_coord_object <- castle_data$dataframe[which(castle_data$dataframe$z == player$floor & castle_data$datafram$object %in% c("DS","\U2395")),1:4]
      coord <- get_coord_object[1:3]
      object <- get_coord_object[4]
      castle_data$castle[coord[[1]],coord[[2]],coord[[3]]] <- "\U2395"
    }
    display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
    #Information about the number of monsters left to defeat to progress
    switch (object[[1]],
            "DS" = {cat("The location of the stairs has been revealed!")
              },
            "\U2395" = {cat("The location of the exit has been revealed!")
              },
            cat(sprintf("Defeat all monsters to unlock the door: %s left.", player$monster_threshold))
    )
    #Information regarding the number of steps left until zombie can move if player has
    #activated their crystal ball
    if(player$zombie_halt > 0){
      new_line(2)
      cat(sprintf("Number of steps before zombie can move: %s left.",player$zombie_halt))
      }
    new_line(2)
    #Get time before the game updates
    player$stimulus_time <- as.numeric(Sys.time())
    player_action <- read_console_player_movement_action()
    if(player_action == "i"){
      player$encountered_object <- "inventory"
    }else if(player_action == "q"){
      game_status <- "quit"
    }else{
      #Get the movement coordinate which is the sum of the player coordinate and the vector in the movement dictionary
      #corresponding to the valid player action
      player$movement_coordinate <- player$current_coordinate + player$movement_dict[[player_action]]
      #Allow player to appear on the opposite end of the grid if the coordinate is out of bounds
      if(length(dimension <- which(player$movement_coordinate[1:2] %in% c(0,castle_data$castle_length + 1))) > 0){
        min <- 0
        max <- castle_data$castle_length + 1
        bound <- which(c(min,max) == player$movement_coordinate[dimension])
        if(length(bound) > 0){
          if(bound == 1){
            player$movement_coordinate[dimension] <- max - 1
            }else{
              player$movement_coordinate[dimension] <- max %% max + 1
          }
        }
      }
      
      #Get changed dimension
      player$coordinate_difference <- (player$movement_coordinate - player$current_coordinate)[1:2]
      player$changed_dimension <- which(player$coordinate_difference != 0)
      player$coordinate_difference  <- player$coordinate_difference[player$changed_dimension]
      #Get the encountered object, depending on whether the object is an empty space, or zombie, or needs a dataframe search
      player$encountered_object <- castle_data$castle[player$movement_coordinate]
      if(player$encountered_object %in% c("\U1F6AA","\U2395")){
        player$castle_dataframe_row <- which(castle_data$dataframe$x == player$movement_coordinate[1] & castle_data$dataframe$y == player$movement_coordinate[2] & castle_data$dataframe$z == player$movement_coordinate[3])
        number <- castle_data$dataframe[player$castle_dataframe_row,"hp"]
        if(number > 0){
          if(castle_data$dataframe[player$castle_dataframe_row,"object"] %in% c("\U1F479", "\U1F9DB","\U1F409")){
            player$encountered_object <- "monster"
          }else if(castle_data$dataframe[player$castle_dataframe_row,"object"] == "DS"){
            player$encountered_object <- "next level"
          }else{
            player$encountered_object <- castle_data$dataframe[player$castle_dataframe_row, "object"]
          }
        }
      }
    }
    if(game_status == "play"){
      #Update player and zombie location if encountered object is an empty space or zombie
      if(player$encountered_object %in% c("","\U1F9DF","\U1F6AA")){
        #Clear current coordinate if it contains player
        if(castle_data$castle[player$current_coordinate] == "\U1F93A"){
          castle_data$castle[player$current_coordinate] <- ""
        }
        if(player$encountered_object %in% c("","\U1F6AA")){
          #Add emoji
          if(player$encountered_object == ""){
            castle_data$castle[player$movement_coordinate] <- "\U1F93A"
          }
          #Update coordinate
          player$current_coordinate <- player$movement_coordinate
          #Small delay to ensure no divide by 0 error.
          Sys.sleep(0.01)
          #Get epoch time after object movement and get velocity
          player$current_game_update_time <- as.numeric(Sys.time())
          player$calculate_player_velocity()
          #Zombie event
          if(player$zombie_halt == 0){
            event_output <- zombie$pathfinding(castle_data = castle_data,player = player)
            castle_data <- event_output[1:3]
          }else{
            player$zombie_halt <- player$zombie_halt - 1
          }
        }else if(player$encountered_object == "\U1F9DF"){
          player$current_coordinate <- player$movement_coordinate
          zombie$distance_to_player <- 0
        }
      }else{
        if(!(player$floor == player$total_floors & player$encountered_object == "\U2395")){
          #Look into dataframe to see what encountered object is supposed to be
          #Switch statement for events
          switch(player$encountered_object,
                 "\U1F9DA"= {event_output <- fairy_event(castle_data = castle_data,player = player)
                 },
                 "\U1F9DE"= {event_output <- genie_event(castle_data = castle_data,player = player)
                 },
                 "monster"= {
                   #Obtain the monster unicode that needs to be displayed - Vampire or Ogre
                   player$encountered_object <- castle_data$dataframe[player$castle_dataframe_row,"object"]
                   event_output <- monster_event(castle_data = castle_data,player = player)
                 },
                 "next level" =  {
                   if(!(castle_data$dataframe[player$castle_dataframe_row,"object"] == "\U2395")){
                     #Get Unicode
                     player$encountered_object <- castle_data$dataframe[player$castle_dataframe_row,"object"]
                     castle_data <- player$move_to_new_floor_event(castle_data = castle_data)
                     if(zombie$current_coordinate[3] != player$current_coordinate[3]){
                       event_output <- zombie$move_to_new_floor_event(castle_data = castle_data,player = player)
                     }else{
                       event_output <- c(castle_data,player)
                     }
                   }
                 },
                 "AS" = {
                   event_output <- upstairs_event(castle_data = castle_data,player = player)
                 },
                 "inventory" = {
                   event_output <- item_inventory(castle_data = castle_data,player = player,game_sequence = "free movement")
                 },
                 "\U1F9DD" = {
                   event_output <- merchant$menu(castle_data = castle_data,player = player,game_sequence = "merchant")
                 }
          )
          #Collect the outputs from each function
          castle_data <- event_output[1:3]
          player <- event_output[[4]]
        }
      }
    }
    }
  #if/else statement to continue or quit when they die
  if(game_status != "quit"){
    if(zombie$distance_to_player == 0){
      player$hp <- 0
      prompt <- "You were eaten by the zombie" 
    }else if(player$encountered_object=="\U2395" & player$floor == player$total_floors){
      display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
      prompt <- "You found the exit!"
    }
    display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
    cat(prompt)
    new_line(2)
    #Retry 
    iteration <<- iteration + 1
    read_console_try_again_action()
  }else{
    new_line(2)
    cat("Thank you for playing Castle Descent!")
  }
  
}

