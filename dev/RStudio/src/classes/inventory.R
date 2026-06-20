# TODO: Refactor

item_inventory <- function(castle_data,player,game_sequence){
  player_action <- ""
  object <- player$observable_item_inventory[2]
  healing_items <- c("\U1F371","\U1F9EA")
  while(!(player_action == "e")){
    new_line(50)
    cat("Inventory")
    new_line(1)
    cat("___________________________________________")
    new_line(2)
    cat(player$observable_item_inventory)
    new_line(1)
    cat("___________________________________________")
    new_line(2)
    #Item descriptions
    item_descriptions(object = object, cost = "no")
    new_line(1)
    cat("___________________________________________")
    #Show number in inventory
    new_line(2)
    if(!(object == "")){
      cat(sprintf("Number in inventory: %s",player$hidden_item_inventory[[object]]))
    }
    new_line(2)
    #Player input
    player_action <- read_console_player_menu_action(game_sequence = "inventory")
    #Move cursor
    if(player_action %in% c("a","d")){
      player <- cursor$move_cursor(player,player_action,game_sequence = "inventory")
      object <- player$observable_item_inventory[cursor$position + 1]
      #Clear current player_action
      player_action <- ""
    }else if(player_action == "s"){
      #Use valid item
      if(game_sequence == "free movement" | game_sequence == "battle" & object %in% healing_items){
        player <- use_item(player = player, object = object)
        #Check if objects are empty
        if(0 %in% player$hidden_item_inventory){
          player$reset_inventory()
          }
        player_action <- ""
        object <- player$observable_item_inventory[cursor$position + 1]
        }else{
          if(!(object == "")){
            new_line(50)
            cat("Inventory")
            new_line(1)
            cat("___________________________________________")
            new_line(2)
            cat(player$observable_item_inventory)
            new_line(1)
            cat("___________________________________________")
            new_line(2)
            cat("Cannot use this item in battle.")
            player_action <- ""
            Sys.sleep(1)
            }
      }
    }
    }
  #Erase cursor and add back to slot 1
  player$observable_item_inventory[which(player$observable_item_inventory == "\U2771")] <- ""
  #Reset cursor position
  cursor$position <- 1
  player$observable_item_inventory[cursor$position] <- "\U2771"
  inventory_output <- c(castle_data,player)
  return(inventory_output)
  }
  

use_item <- function(player,object){
  new_line(50)
  #Reduce number of items left
  player$hidden_item_inventory[[object]] <- player$hidden_item_inventory[[object]] - 1
  new_line(50)
  cat("Inventory")
  new_line(1)
  cat("___________________________________________")
  new_line(2)
  cat(player$observable_item_inventory)
  new_line(1)
  cat("___________________________________________")
  new_line(2)
  #Adding item effects to player
  switch(object,
         "\U1F52E" = {
           player$zombie_halt <- player$zombie_halt + sample(10:20,1)
           cat(sprintf("Zombie halted for %s steps.",player$zombie_halt))
           },
         "\U1F371" = {
           if(player$hp == 100){
             cat("You are already at full health.")
             #Add back item
             player$hidden_item_inventory[[object]] <- player$hidden_item_inventory[[object]] + 1
             }else if(player$hp + 20 > 100){
               restore <- player$hp + 20 + (100 - (player$hp + 20))
               cat(sprintf("Your HP increased by %s points",abs(100 - player$hp)))
               player$hp <- restore
             }else{
               cat("Your HP increased by 20 points.")
               player$hp <- player$hp + 20
             }
           },
         "\U0001f50e" = {
           if(player$monster_threshold == 0){
             cat("The door has already been revealed.")
             #Add back item
             player$hidden_item_inventory[[object]] <- player$hidden_item_inventory[[object]] + 1
           }else{
             player$monster_threshold <- 0
             cat("The door has been revealed.")
             }
           },
         "\U1F9EA" = {
           if(player$mana == 100){
             cat("You mana is already maxed out.")
             #Add back item
             player$hidden_item_inventory[[object]] <- player$hidden_item_inventory[[object]] + 1
             }else if(player$mana + 30 > 100){
               restore <- player$mana + 30 + (100 - (player$mana + 30))
               cat(sprintf("Your mana increased by %s points",abs(100 - player$mana)))
               player$mana <- restore
             }else{
               cat("Your mana increased by 30 points.")
               player$mana <- player$mana + 30
               }
           }
         )
         
  Sys.sleep(1)
  return(player)
  
}

