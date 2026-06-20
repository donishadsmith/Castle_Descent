# TODO: Refactor
from display import new_line
item_costs={
                              u'\U0001F52E': 50,
                              u'\U0001F371': 10,
                              u'\U0001F50E': 500,
                              u'\U0001F9EA': 40,
                          }
def item_description(object, cost):
    if cost == 'yes':
        prompt = f'Cost: {item_costs[object]}'
    else:
        prompt = ''
                
    new_line(1)

    match object:
            case u'\U0001F52E': 
                print('Crystal Ball')
                print('------------')
                new_line(1)
                print('Temporarily halts zombie movement.')
                new_line(1)
                print(prompt)
            case u'\U0001F371': 
                print('Bento Box')
                print('---------')
                new_line(1)
                print('Restores 20 HP.')
                new_line(1)
                print(prompt)                   
            case u'\U0001F50E':
                print('Magnifying Glass')
                print('----------------')
                new_line(1)
                print('Reveals the door leading to stairs or exit.')
                new_line(1)
                print(prompt)
            case u'\U0001F9EA':
                print('Mana Potion')
                print('-----------')
                new_line(1)
                print('Restores 30 mana.')
                new_line(1)
                print(prompt)
            case _:
                print('')

if __name__ == '__main__':
    print("You must run 'python3 start_game.py' to play Castle Descent.")