from display import new_line

item_costs = {
    "\U0001f52e": 50,
    "\U0001f371": 10,
    "\U0001f50e": 500,
    "\U0001f9ea": 40,
}


def item_description(object, cost):
    if cost == "yes":
        prompt = f"Cost: {item_costs[object]}"
    else:
        prompt = ""

    new_line(1)

    match object:
        case "\U0001f52e":
            print("Crystal Ball")
            print("------------")
            new_line(1)
            print("Temporarily halts zombie movement.")
            new_line(1)
            print(prompt)
        case "\U0001f371":
            print("Bento Box")
            print("---------")
            new_line(1)
            print("Restores 20 HP.")
            new_line(1)
            print(prompt)
        case "\U0001f50e":
            print("Magnifying Glass")
            print("----------------")
            new_line(1)
            print("Reveals the door leading to stairs or exit.")
            new_line(1)
            print(prompt)
        case "\U0001f9ea":
            print("Mana Potion")
            print("-----------")
            new_line(1)
            print("Restores 30 mana.")
            new_line(1)
            print(prompt)
        case _:
            print("")


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
