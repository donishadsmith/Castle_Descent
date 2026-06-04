# For checking width of unicode characters
# import wcwidth
def new_line(num):
    for _ in range(num):
        print("\n")


def display_array(
    castle,
    game_sequence,
    player=None,
    monster_hp=None,
    hp=None,
    mana=None,
    money=None,
    floor=None,
):
    new_line(50)
    if game_sequence in ["non-battle", "battle"]:
        hp = player.hp
        mana = player.mana
        money = player.money
        floor = player.floor
    new_line(1)
    print(f"Floor {floor + 1} of {len(castle)}")
    new_line(1)
    # print('\n'.join(['  '.join(['{:^{}}'.format(cell,wcwidth.wcwidth(u'\U0001f6aa')) if cell == '' else cell for cell in row]) for row in castle[floor]]))
    print(
        "\n".join(
            [
                "  ".join(
                    ["{:^{}}".format(cell, 2) if cell == "" else cell for cell in row]
                )
                for row in castle[floor]
            ]
        )
    )
    new_line(1)
    if not game_sequence == "battle":
        print(f"HP: {hp}% | Mana: {mana}% | \U0001f4b0: {money}")
    else:
        print(
            f"HP: {hp}% | Mana: {mana}% | \U0001f4b0: {money} | Monster HP: {monster_hp}"
        )
    new_line(1)


if __name__ == "__main__":
    print("You must run 'python3 start_game.py' to play Castle Descent.")
