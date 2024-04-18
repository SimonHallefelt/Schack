# data.txt will contain player turn a score and the board for every turn of every match in raw_data.txt

def get_new_bit_board():
    wk = 1 << 4
    wq = 1 << 3
    wr = (1 << 7) | (1 << 0)
    wn = (1 << 6) | (1 << 1)
    wb = (1 << 5) | (1 << 2)
    wp = 0b11111111 << 8
    bk = 1 << 60
    bq = 1 << 59
    br = (1 << 63) | (1 << 56)
    bn = (1 << 62) | (1 << 57)
    bb = (1 << 61) | (1 << 58)
    bp = 0b11111111 << 48
    wpieces = wk | wq | wr | wn | wb | wp
    bpieces = bk | bq | br | bn | bb | bp
    return {
        '6': wk,
        '5': wq,
        '4': wr,
        '3': wb,
        '2': wn,
        '1': wp,
        '-6': bk,
        '-5': bq,
        '-4': br,
        '-3': bb,
        '-2': bn,
        '-1': bp,
        'wpieces': wpieces,
        'bpieces': bpieces
    }

def print_board(board, turn):
    for i in range(7, -1, -1):
        for j in range(8):
            if board['6'] & (1 << (i * 8 + j)):
                print('K', end=' ')
            elif board['5'] & (1 << (i * 8 + j)):
                print('Q', end=' ')
            elif board['4'] & (1 << (i * 8 + j)):
                print('R', end=' ')
            elif board['3'] & (1 << (i * 8 + j)):
                print('B', end=' ')
            elif board['2'] & (1 << (i * 8 + j)):
                print('N', end=' ')
            elif board['1'] & (1 << (i * 8 + j)):
                print('P', end=' ')
            elif board['-6'] & (1 << (i * 8 + j)):
                print('k', end=' ')
            elif board['-5'] & (1 << (i * 8 + j)):
                print('q', end=' ')
            elif board['-4'] & (1 << (i * 8 + j)):
                print('r', end=' ')
            elif board['-3'] & (1 << (i * 8 + j)):
                print('b', end=' ')
            elif board['-2'] & (1 << (i * 8 + j)):
                print('n', end=' ')
            elif board['-1'] & (1 << (i * 8 + j)):
                print('p', end=' ')
            else:
                print('.', end=' ')
        print()
    print()



def make_move(board, piece, start, end, promotion):
    if piece > 0:
        board['wpieces'] &= ~(1 << (start[0] + 8 * start[1]))
        board['wpieces'] |= (1 << (end[0] + 8 * end[1]))
        board['bpieces'] &= ~(1 << (end[0] + 8 * end[1]))
    else:
        board['bpieces'] &= ~(1 << (start[0] + 8 * start[1]))
        board['bpieces'] |= (1 << (end[0] + 8 * end[1]))
        board['wpieces'] &= ~(1 << (end[0] + 8 * end[1]))

    if abs(piece) == 1 and abs(start[0] - end[0]) == 1 and board[str(piece*-1)] & (1 << (end[0] + 8 * end[1])) == 0:
        board['bpieces'] &= ~(1 << (end[0] + 8 * start[1]))
        board['wpieces'] &= ~(1 << (end[0] + 8 * start[1]))
        board[str(piece*-1)] &= ~(1 << (end[0] + 8 * start[1]))
    elif abs(piece) == 6 and abs(start[0] - end[0]) == 2:
        if piece < 0:
            if end[0] < start[0]:
                board['bpieces'] &= ~(1 << (0 + 8 * start[1]))
                board['bpieces'] |= (1 << (3 + 8 * start[1]))
                board['-4'] &= ~(1 << (0 + 8 * start[1]))
                board['-4'] |= (1 << (3 + 8 * start[1]))
            else:
                board['bpieces'] &= ~(1 << (7 + 8 * start[1]))
                board['bpieces'] |= (1 << (5 + 8 * start[1]))
                board['-4'] &= ~(1 << (7 + 8 * start[1]))
                board['-4'] |= (1 << (5 + 8 * start[1]))
        else:
            if end[0] < start[0]:
                board['wpieces'] &= ~(1 << (0 + 8 * start[1]))
                board['wpieces'] |= (1 << (3 + 8 * start[1]))
                board['4'] &= ~(1 << (0 + 8 * start[1]))
                board['4'] |= (1 << (3 + 8 * start[1]))
            else:
                board['wpieces'] &= ~(1 << (7 + 8 * start[1]))
                board['wpieces'] |= (1 << (5 + 8 * start[1]))
                board['4'] &= ~(1 << (7 + 8 * start[1]))
                board['4'] |= (1 << (5 + 8 * start[1]))

    board[str(piece)] &= ~(1 << (start[0] + 8 * start[1]))
    board[str(piece)] |= (1 << (end[0] + 8 * end[1]))

    if promotion:
        board[str(piece)] &= ~(1 << (end[0] + 8 * end[1]))
        board[str(promotion)] |= (1 << (end[0] + 8 * end[1]))
    
    if piece > 0:
        board['-5'] &= ~(1 << (end[0] + 8 * end[1]))
        board['-4'] &= ~(1 << (end[0] + 8 * end[1]))
        board['-3'] &= ~(1 << (end[0] + 8 * end[1]))
        board['-2'] &= ~(1 << (end[0] + 8 * end[1]))
        board['-1'] &= ~(1 << (end[0] + 8 * end[1]))
    else:
        board['5'] &= ~(1 << (end[0] + 8 * end[1]))
        board['4'] &= ~(1 << (end[0] + 8 * end[1]))
        board['3'] &= ~(1 << (end[0] + 8 * end[1]))
        board['2'] &= ~(1 << (end[0] + 8 * end[1]))
        board['1'] &= ~(1 << (end[0] + 8 * end[1]))





raw_data = open("data\\raw_data.txt", "r")
matches = raw_data.readlines()

data = []
pieces_lookup = { 'K': 6, 'Q': 5, 'R': 4, 'B': 3, 'N': 2, 'P': 1 }
columns_lookup = { 'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 'g': 6, 'h': 7 }
for match in matches:
    turn = 1
    match = match.split()
    result = match[0]
    player1 = match[1]
    player2 = match[2]
    moves = match[3:]
    total_moves = len(moves)

    bit_board = get_new_bit_board()
    # print_board(bit_board, turn)

    for i, move in enumerate(moves):
        piece = 0
        start = [0,0]
        end = [0,0]
        promotion = 0
        if move[0] == 'O':
            piece = 6
            r = 0
            if turn == -1:
                r = 7
            if move == 'O-O':
                start = [4, r]
                end = [6, r]
            else:
                start = [4, r]
                end = [2, r]
        else:
            piece = pieces_lookup[move[0]]
            start = [columns_lookup[move[1]], int(move[2])]
            end = [columns_lookup[move[4]], int(move[5])]
            if len(move) > 6:
                promotion = pieces_lookup[move[7]]
        
        make_move(bit_board, piece*turn, start, end, promotion)
        # print(move)
        # print_board(bit_board, turn)
        turn *= -1

        score = 0.0
        if result == '3':
            score = 0.5
        elif result == '1':
            score = 0.6 + 0.2 * (i / total_moves)
            if i == total_moves - 1:
                score = 1.0
        else:
            score = 0.4 - 0.2 * (i / total_moves)
            if i == total_moves - 1:
                score = 0.0

        data.append((turn, score, bit_board))

    # print_board(bit_board, turn)
    


file = open("data\\data.txt", "w")
for d in data:
    file.write(str(d[0]) + " " + str(d[1]))
    for value in d[2].values():
        file.write(" " + str(value))
    file.write("\n")

