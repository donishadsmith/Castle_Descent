# TODO: Refactor

#Function for genie event
genie_event <- function(castle_data,player){
  castle_data$castle[player$movement_coordinate] <- player$encountered_object
  display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
  cat("You encountered a genie!")
  player_choice <- ""
  player_action <- ""
  if("Reduce Mana Cost" %in% player$menus[["genie"]]){
    while(!player_action == "s"){
      display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
      cat("___________________________________________________")
      new_line(1)
      cat(player$menus[["genie"]])
      new_line(1)
      cat("___________________________________________________")
      new_line(2)
      player_action <- read_console_player_menu_action(game_sequence = "genie")
      #Move cursor
      if(player_action %in% c("a","d")){
        player <- cursor$move_cursor(player,player_action,game_sequence = "genie")
      }else{
        player_choice <- player$menus[["genie"]][cursor$position + 1]
      }
    }
  }else{
    player_choice = "Increase Attack"
  }
  display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
  if(player_choice == "Increase Attack"){
    attack_increase <- sample(1:5,1)
    cat(sprintf("Your Attack range and Enchanced Attack range increased by %s points.",attack_increase))
    player$attack_range <- player$attack_range + attack_increase
    player$enhanced_attack_range <- player$enhanced_attack_range + attack_increase
    new_line(2)
    cat(sprintf("New Attack range: %s:%s",min(player$attack_range),max(player$attack_range)))
    new_line(2)
    cat(sprintf("New Enchanced Attack range: %s:%s",min(player$enhanced_attack_range),max(player$enhanced_attack_range)))
  }else if(player_choice == "Reduce Mana Cost"){
    decrease_mana_cost <- sample(1:5,1)
    player$mana_cost <- player$mana_cost - decrease_mana_cost
    if(player$mana_cost < 1){
      player$mana_cost <- 1
      #Reducing mana no longer available
      player$menus[["genie"]] <- player$menus[["genie"]][1:2]
    }
    player$menus[["battle"]][4] <- sprintf("Enchanced Attack(%s mana)", player$mana_cost)
    cat(sprintf("Your Enhanced Attack now costs %s mana",player$mana_cost))
  }
  #Adding pause so player can read info
  Sys.sleep(1.5)
  #Adding back door and a zero in dataframe
  castle_data$castle[player$movement_coordinate] <- "\U1F6AA"
  #Add zero to dataframe to deactivate event
  castle_data$dataframe[player$castle_dataframe_row,"hp"] <- 0
  #Erase cursor and add back to slot 1
  player$menus[["genie"]][which(player$menus[["genie"]] == "\U2771")] <- ""
  #Reset cursor position
  cursor$position <- 1
  player$menus[["genie"]][cursor$position] <- "\U2771"
  inventory_output <- c(castle_data,player)
  #Return information
  genie_event_output <- c(castle_data,player)
  return(genie_event_output)
}
