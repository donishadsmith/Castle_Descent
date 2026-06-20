# TODO: Refactor
#Descriptions for in game items
item_descriptions <- function(object,cost){
  if(cost == "yes"){
   cost_prompt <- sprintf("Cost: %s", cost <- merchant$item_costs[[object]])
  }else{
   cost_prompt <- ""
  }
  switch(object,
         "\U1F52E" = {
           cat("Crystal Ball")
           new_line(1)
           cat("------------")
           new_line(2)
           cat("Temporarily halts zombie movement.")
           new_line(2)
           cat(cost_prompt)
         },
         "\U1F371" = {
           cat("Bento Box")
           new_line(1)
           cat("---------")
           new_line(2)
           cat("Heals 20 hp.")
           new_line(2)
           cat(cost_prompt)
         },
         "\U0001f50e" = {
           cat("Magnifying Glass")
           new_line(1)
           cat("----------------")
           new_line(2)
           cat("Reveals the door leading to stairs or exit.")
           new_line(2)
           cat(cost_prompt)
         },
         "\U1F9EA" = {
           cat("Mana Potion")
           new_line(1)
           cat("-----------")
           new_line(2)
           cat("Restores 30 mana.")
           new_line(2)
           cat(cost_prompt)
         },
         cat(cost_prompt)
  )
 
}
  