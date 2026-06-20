# TODO: Refactor

#Zombie class contains the necessary attributes to keep track of zombie coordinates
#and methods to allow zombie to move
zombie_class <- setRefClass("zombie_info", 
                            fields = list(movement_dict = "list",
                                          current_coordinate = "matrix",
                                          distance_to_player= "numeric",
                                          floor = "numeric"),
                            methods = list(
                              chebyshev_distance = function(a,b){
                                distance <- max(abs(a - b))
                                return(distance)
                              },
                              pathfinding = function(castle_data,player){
                                #Using the current coordinate, get list of for possible
                                movement_vector <- c()
                                for(movement in movement_dict){
                                  possible_coordinate <- current_coordinate + movement
                                  #If coordinate is not out of bounds and is empty or contains the player, it is a possible coordinate to move to
                                  if(!(length(which(possible_coordinate[1:2] %in% c(0, castle_data$castle_length + 1) > 0)))){
                                    if(castle_data$castle[possible_coordinate] == ""| castle_data$castle[possible_coordinate] == "\U1F93A"){
                                      movement_vector <- c(movement_vector,list(possible_coordinate))
                                    }
                                  }
                                }
                                #See if player coordinate is in the movement_vector
                                logic_vector <- c()
                                for(possible_coordinate in movement_vector){
                                  logic_vector <- c(logic_vector,possible_coordinate == player$current_coordinate)
                                }
                                #If player coordinate isn't in movement vector
                                if(!(T %in% all(logic_vector))){
                                  #If it is within a certain range it starts to predict
                                  dynamic_t <- chebyshev_distance(current_coordinate,player$current_coordinate)/player$max_velocity
                                  if(player$changed_dimension == 1){
                                    player$current_velocity <- c(player$current_velocity,0,0)
                                    player$acceleration <- c(player$acceleration,0,0)
                                  }else{
                                    player$current_velocity <- c(0,player$current_velocity,0)
                                    player$acceleration <- c(0,player$acceleration,0)
                                    }
                                  displacement <- player$current_velocity*dynamic_t + (player$acceleration*(dynamic_t)^2)/2
                                  predicted_player_position <- round(player$current_coordinate + displacement,0)
                                  
                                  distance_to_predicted_player_position <- c()
                                  for(possible_coordinate in movement_vector){
                                    distance_to_predicted_player_position <- c(distance_to_predicted_player_position,chebyshev_distance(possible_coordinate,predicted_player_position))
                                  }
                                  current_coordinate <<-  movement_vector[[which(distance_to_predicted_player_position == min(distance_to_predicted_player_position))[1]]]
                                  #If player coordinate is in movement vector, zombie moves to the coordinate
                                  }else{
                                    current_coordinate <<- player$current_coordinate
                                    }
                                #Erase zombie from old location and add to new location
                                castle_data$castle[which(castle_data$castle=="\U1F9DF",arr.ind = T)] <- ""
                                castle_data$castle[current_coordinate] <- "\U1F9DF"
                                #Calculate new distance from zombie to player
                                distance_to_player <<- chebyshev_distance(current_coordinate,player$current_coordinate)
                                #Return information
                                pathfinder_output <- c(castle_data,player)
                                return(pathfinder_output)
                              },
                              #Allow zombie to move to new floor with player
                              move_to_new_floor_event = function(castle_data,player){
                                #Find the coordinate that allows the zombie to be at the greatest Chebyshev distance from the player
                                movable_spaces <- which(castle_data$castle[,,player$floor]=="", arr.ind = T)
                                max_distance <- c()
                                for(row in 1:nrow(movable_spaces)){
                                  max_distance <- c(max_distance,chebyshev_distance(movable_spaces[row,],player$current_coordinate[1:2]))
                                }
                                coord <- movable_spaces[which(max_distance== max(max_distance))[1],]
                                #Erase old zombie location
                                castle_data$castle[which(castle_data$castle=="\U1F9DF", arr.ind = T)] <- ""
                                #Add new zombie location and update initial coordinate,current coordinate, and distance
                                castle_data$castle[,,player$floor][coord[1],coord[2]] <- "\U1F9DF"
                                current_coordinate <<- which(castle_data$castle=="\U1F9DF", arr.ind = T)
                                distance_to_player <<- chebyshev_distance(current_coordinate,player$current_coordinate)
                                #Reset 
                                move_to_new_floor_event_output <- c(castle_data,player)
                                return(move_to_new_floor_event_output)
                              }))
