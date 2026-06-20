# TODO: Refactor

#Function for fairy event
fairy_event <- function(castle_data,player){
  #Add space between previously printed screen and new screen
  castle_data$castle[player$movement_coordinate] <- player$encountered_object
  display_array(castle_data = castle_data,player = player,game_sequence = "non-battle")
  cat("You encountered a fairy!")
  new_line(2)
  #Player mana and hp capped at 100
  if(player$hp == 100 & player$mana == 100){
    cat("Your HP and mana are already full. Come back later.")
    #Add pause to allow player to read information
  }else{
    if(player$hp == 100 & player$mana < 100){
      cat("Your mana was fully restored!.")
      player$mana <- 100
    }else if(player$hp < 100 & player$mana == 100){
      cat("Your HP was fully restored!")
      player$hp <- 100
    }else{
      cat("Your HP and mana were fully restored!")
      player$mana <- 100
      player$hp <- 100
    }
    
    castle_data$dataframe[player$castle_dataframe_row,"hp"] <- 0
  }
  #Add pause to allow player to read information
  Sys.sleep(1.5)
  #Adding back door 
  castle_data$castle[player$movement_coordinate] <- "\U1F6AA"
  if(castle_data$dataframe[player$castle_dataframe_row,"hp"] == 0){
    #Adding zero to dataframe so that fairy event is deactivated
    castle_data$dataframe[player$castle_dataframe_row,"hp"] <- 0
  }
  #Return information
  fairy_event_output <- c(castle_data,player)
  return(fairy_event_output)
}