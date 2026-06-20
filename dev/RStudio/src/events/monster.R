# TODO: Refactor

#Function for monster event
monster_event <- function(castle_data,player){
  #Go to dataframe and extract 
  castle_data$castle[player$movement_coordinate] <- player$encountered_object
  monster_hp <- castle_data$dataframe[player$castle_dataframe_row, "hp"]
  display_array(castle_data = castle_data,player = player,game_sequence = "battle",
                monster_hp = monster_hp)
  cat("You encountered a monster!")
  new_line(2)
  player_choice <- ""
  player_action <- ""
  #While loop so that player can engage with monster unless they decide to run, they faint, or they win
  while(!(player_choice == "Run" | monster_hp == 0 | player$hp == 0)){
    #Player input command for battle menu
    while(!player_action == "s"){
      display_array(castle_data = castle_data,player = player,game_sequence = "battle",
                    monster_hp = monster_hp)
      cat("___________________________________________________")
      new_line(1)
      cat(player$menus[["battle"]])
      new_line(1)
      cat("___________________________________________________")
      new_line(2)
      player_action <- read_console_player_menu_action(game_sequence = "battle")
      #Move cursor
      if(player_action %in% c("a","d")){
        player <- cursor$move_cursor(player,player_action,game_sequence = "battle")
      }else{
        player_choice <- player$menus[["battle"]][cursor$position + 1]
        #Check mana
        if(!player_choice %in% c("Attack","Run", "Inventory")){
          if(player$mana - player$mana_cost < 0){
            display_array(castle_data = castle_data,player = player,game_sequence = "battle",
                          monster_hp = monster_hp)
            cat("___________________________________________________")
            new_line(1)
            cat(player$menus[["battle"]])
            new_line(1)
            cat("___________________________________________________")
            new_line(2)
            cat("You don't have sufficient mana.")
            #Clear player_action
            player_action = ""
            Sys.sleep(1)
          }
        }
      }
    }
    
    if(!player_choice %in% c("Run", "Inventory")){
      event_output <- battle_sequence(castle_data = castle_data,player = player,monster_hp = monster_hp,
                                      player_choice = player_choice)
      castle_data <- event_output[1:3]
      player <- event_output[[4]]
      monster_hp <- castle_data$dataframe[player$castle_dataframe_row,"hp"]
    }else if(player_choice == "Inventory"){
      event_output <- item_inventory(castle_data = castle_data,player = player,game_sequence = "battle")
      player_action <- ""
      player <- event_output[[4]]
    }
    player_action <- ""
  }
  #Add back door
  castle_data$castle[player$movement_coordinate] <- "\U1F6AA"
  #Erase cursor and add back to slot 1
  player$menus[["battle"]][which(player$menus[["battle"]] == "\U2771")] <- ""
  #Reset cursor position
  cursor$position <- 1
  player$menus[["battle"]][cursor$position] <- "\U2771"
  monster_event_output <- c(castle_data,player)
  return(monster_event_output)
}
#Function for monster combat
battle_sequence <- function(castle_data,player,monster_hp,player_choice){
  display_array(castle_data = castle_data,player = player,game_sequence = "battle",
                monster_hp = monster_hp)
  if(player_choice == "Attack"){
    player$attack_power <- sample(player$attack_range,1)
  }else{
    player$attack_power <- sample(player$enhanced_attack_range,1)
    player$mana <- player$mana - player$mana_cost
  }
  cat(sprintf("You dealt %s points of damage",player$attack_power))
  new_line(2)
  #Monster hp set to zero to exit loop if attack > than monster hp
  if(monster_hp - player$attack_power > 0){
    #Update monster hp, if player decides to run, when they encounter monster again, 
    #it will reflect the new hp
    castle_data$dataframe[player$castle_dataframe_row,"hp"] <- monster_hp <-  monster_hp - player$attack_power
    monster_attack <- sample(1:5,1)
    player$hp <- player$hp - monster_attack
    cat(sprintf("Monster dealt %s points of damage",monster_attack))
    if(player$hp <= 0){
      new_line(2)
      #player health set to zero if monster attack > than player hp
      player$hp <- 0
      cat("You died.")
    }
  }else{
    #Money won is 1.5 times the monsters full hp
    win_money = castle_data$dataframe[player$castle_dataframe_row,"win_money"] 
    #Add zero to dataframe to ensure that event is not triggered
    castle_data$dataframe[player$castle_dataframe_row,"hp"] <- monster_hp <- 0
    #Random Drop
    random_drop <- sample(c(names(player$hidden_item_inventory),""), size = 1, prob = c(0.125,0.125,0.025,0.125,0.60))
    if(!random_drop == ""){
      player$hidden_item_inventory[[random_drop]] <- player$hidden_item_inventory[[random_drop]] + 1
      #If it is not in inventory, loop through available spaces to add it too
      if(length(which(player$observable_item_inventory[seq(2,8,2)] == random_drop)) == 0){
        free_space <- which(player$observable_item_inventory[seq(2,8,2)] == "")
        player$observable_item_inventory[seq(2,8,2)][free_space[1]] <- random_drop
      }
      prompt <- sprintf("Money: %s\n\nItem Drop: %s",win_money,random_drop)
    }else{
      prompt <- sprintf("Money: %s\n",win_money)
    }
    player$money = player$money + win_money
    cat("The monster fainted.")
    new_line(2)
    cat(prompt)
    #Set monster hp to 0 in dataframe
    castle_data$dataframe[player$castle_dataframe_row,"hp"] <- 0
    if(player$monster_threshold > 0){
      player$monster_threshold <- player$monster_threshold - 1
    }
  }
  Sys.sleep(1.5)
  monster_combat_event_output <- c(castle_data,player)
  return(monster_combat_event_output)
}