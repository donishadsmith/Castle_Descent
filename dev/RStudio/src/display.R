# TODO: Refactor
display_array <- function(castle_data,player,floor,total_floors,mana,hp,money,game_sequence,monster_hp){
  new_line(50)
  if(!game_sequence == "next level"){
    floor <- player$floor 
    total_floors <- player$total_floors
    hp <- player$hp
    mana <- player$mana
    money <- player$money
  }
  cat(sprintf("Floor %s of %s",floor,total_floors))
  new_line(2)
  print(castle_data$castle[,,floor],quote = F)
  new_line(1)
  if(game_sequence %in% c("non-battle", "next level")){
    #Display Health information and money
    cat(sprintf("HP: %s %s Mana: %s %s %s: %s",paste0(hp,"%"),"|",paste0(mana,"%"), "|",money_unicode <- "\U1F4B0", money))
  }else{
    cat(sprintf("HP: %s %s Mana: %s %s Monster HP: %s",paste0(hp,"%"),"|",paste0(mana,"%")
                ,"|",monster_hp))
  }

  new_line(2)

}

new_line <- function(num){
  cat(rep("\n", num))
}