# TODO: Refactor

#Player class keeps important information about player such as health and current position
player_class <-  setRefClass("player_info", 
                             fields = list(hp = "numeric",
                                           mana = "numeric",
                                           current_coordinate = "matrix",
                                           money = "numeric",
                                           movement_dict = "list",
                                           hidden_item_inventory = "list",
                                           menus = "list",
                                           mana_cost = "numeric",
                                           observable_item_inventory = "matrix",
                                           encountered_object = "character",
                                           movement_coordinate = "matrix",
                                           attack_range = "numeric",
                                           attack_power = "numeric",
                                           enhanced_attack_range = "numeric",
                                           floor = "numeric",
                                           castle_dataframe_row = "numeric",
                                           #Adding incentive to kill monsters. If player kills a certain number of monsters.
                                           #Stairs/exit is revealed
                                           monster_threshold = "numeric",
                                           zombie_halt = "numeric",
                                           total_floors = "numeric",
                                           max_velocity = "numeric",
                                           stimulus_time = "numeric",
                                           previous_game_update_time = "numeric",
                                           current_game_update_time = "numeric",
                                           previous_velocity = "numeric",
                                           current_velocity = "numeric",
                                           acceleration = "numeric",
                                           coordinate_difference = "numeric",
                                           changed_dimension = "numeric"),
                             methods = list(
                               calculate_player_velocity = function(){
                                 #Change in grid position always equals 1
                                 current_velocity <<- (coordinate_difference/abs(coordinate_difference))/(current_game_update_time - stimulus_time)
                                 acceleration <<- (current_velocity - previous_velocity)/(current_game_update_time - previous_game_update_time)
                                 previous_velocity <<- current_velocity
                                 previous_game_update_time <<- current_game_update_time
                                 if (abs(current_velocity) > abs(max_velocity)){
                                   max_velocity <<- current_velocity
                                 }
                               },
                               move_to_new_floor_event = function(castle_data){
                                 #Get functions from local environment
                                 new_line <- get("new_line", envir = castle_descent)
                                 display_array <- get("display_array", envir = castle_descent)
                                 display_array(castle_data = castle_data,floor = floor,
                                               total_floors = total_floors,
                                               game_sequence = "next level",
                                               hp = hp,
                                               mana = mana,
                                               money = money)
                                 if(monster_threshold > 0){
                                   cat("You must defeat all monsters on this floor to progress.")
                                 }else{
                                   cat(sprintf("You can now advance to floor %s !", floor + 1))
                                   #Erase player
                                   castle_data$castle[current_coordinate] <- ""
                                   #Add 1 to player"s current z-position
                                   floor <<- floor + 1
                                   #Reset the monster threshold
                                   monster_threshold <<- round(length(which(castle_data$dataframe["z"] == floor & castle_data$dataframe["object"]=="\U1F479")),0)
                                   current_coordinate[3] <<- floor
                                   castle_data$castle[current_coordinate] <- "\U1F93A"
                                  }
                                 
                                 
                                 Sys.sleep(1)
                                 #Return information
                                 return(castle_data)
                               },
                               reset_inventory = function(){
                                 #Get items in observable inventory
                                 current_items <- observable_item_inventory[which(!observable_item_inventory %in% c("","\U2771"))]
                                 #Keep items that are greater than 0 in the hidden inventory
                                 current_items <- names(which(hidden_item_inventory[current_items] > 0))
                                 #Eliminate items in observable inventory
                                 observable_item_inventory[c(2,4,6,8)] <<- ""
                                 #Add back items with quantity greater than 0
                                 if(length(current_items) > 0){
                                   observable_item_inventory[seq(2,length(current_items)*2,2)] <<- current_items
                                 }
                               }
                             ))


Player <-  setRefClass(
  "Class for player specific variables and functions",
  fields = list(
    status = "numeric"
    
  ))