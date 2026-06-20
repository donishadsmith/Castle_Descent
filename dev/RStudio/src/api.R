# TODO: Refactor

#Functions for the API
#Use RStudio"s API to dynamically read R's Terminal
read_console_try_again_action <- function() {
  cat("Want to play again? Yes(y) or No(n)?")
  #Loop will continue until player inputs valid response
  while(rstudioapi::isAvailable()){
    player_action <- tolower(rstudioapi::getConsoleEditorContext()$contents)
    if(player_action %in% c("y")){
      #Needed to clear console
      rstudioapi::sendToConsole("",execute = F)
      start_game()
      } else if(player_action %in% c("no","n")){
      new_line(2)
      cat("Thank you for playing Castle Descent!")
      return(noquote(""))
      }
    #Needed so that player can escape game by pressing ctrl + c.
    #It suspends execution of R expressions every n seconds
    rstudioapi::sendToConsole("",execute = F)
    Sys.sleep(0.2)
  }
  }

read_console_player_movement_action <- function() {
  cat("w(up), a(left), s(down), d(right), inventory(i), quit(q): ")
  while(rstudioapi::isAvailable()) {
    player_action <- tolower(rstudioapi::getConsoleEditorContext()$contents)
    
    if(player_action %in% c("w","a","s","d","i","q")) {
      rstudioapi::sendToConsole("",execute = F)
      return(player_action)
      }
    rstudioapi::sendToConsole("",execute = F)
    Sys.sleep(0.2)
  }
  }

read_console_player_menu_action <- function(game_sequence){
  if(game_sequence %in% c("battle","genie")){
    prompt = "a (left), d(right), select (s): "
    valid_actions = c("a","d","s")
  }else{
    prompt = "a (left), d(right), select(s), exit(e): "
    valid_actions = c("a","d","s","e")
  }
  cat(prompt)
  while(rstudioapi::isAvailable()){
    player_action <- tolower(rstudioapi::getConsoleEditorContext()$contents)
    if(player_action %in% valid_actions) {
      rstudioapi::sendToConsole("",execute = F)
      return(player_action)
      }
    rstudioapi::sendToConsole("",execute = F)
    Sys.sleep(0.2)
  }
  }
