
'''
#called by rust

#reads the file "decks & target positions" and "current model"

#returns "decks & predicted position"
import time
time.sleep(5)

print("helo")




def get_ml_decks_unmutated():


    fobj = open("mldecks_no_mutations.json", "rb")
    data = json.load(fobj)


    length = len(data)

    length = 10
    testlength = 0


    train_inputs = np.zeros((length-testlength, 1899))
    train_labels = np.zeros((length-testlength, 1899))

    test_inputs  = np.zeros((testlength, 1899))
    test_labels  = np.zeros((testlength, 1899))


    #print(train_inputs.shape)
    #print(train_labels.shape)
    #print(test_inputs.shape)
    #print(test_labels.shape)


    testleft = testlength


    for deckid in range(length):


        if testleft != 0:

            for card in data[deckid][0]:
                cardid = card[0]
                cardscore = card[1]

                #print(data[deckid][0])
                #print(cardid)

                test_inputs[deckid][cardid] = cardscore
                test_labels[deckid][cardid] = cardscore

            testleft = testleft -1

        else:

            for card in data[deckid][0]:

                cardid = card[0]
                cardscore = card[1]

                train_inputs[deckid-testlength][cardid] = cardscore
                train_labels[deckid-testlength][cardid] = cardscore

    return ( train_inputs, train_labels, test_inputs, test_labels)


'''