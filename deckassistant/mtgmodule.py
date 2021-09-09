

#load the file that has the list of card names in standard
import json
import math
import numpy as np

fobj = open("standardalphabet.json", "rb")
alphabet = json.load(fobj)




def cardidtoname(cardid: int):

    return alphabet[cardid]



def cardnametoid(cardname: str ):

    return alphabet.index(cardname)


def standardlength():

    return len( alphabet )


#given a onehot vector convert it to a list of cards
def onehottocards(onehot: list):

    toreturn = []

    for cardid in range( len(onehot)  ):

        predictionvalue = onehot[cardid]

        amountofcard = round( predictionvalue * 4. )

        for x in range( amountofcard  ):

            toreturn.append(  cardidtoname(cardid) )

    return toreturn


#get the cards in order, up to 30
def onehottocardorder(onehot: list):

    toreturn = []

    for index in np.argsort(onehot):
        toreturn.append( cardidtoname(index) )
    
    #truncate

    toreturn.reverse()
    del toreturn[25:]


    return toreturn


#given a list of cards, convert it into a onehot vector
def cardstoonehot(cardnames: list):

    toreturn = np.zeros( standardlength()  )

    for cardname in cardnames:

        cardid = cardnametoid(cardname)

        toreturn[cardid] += 0.25

    return toreturn


print("imported mtg module")


""" onehot = np.zeros( standardlength()  )
onehot[1] += 0.25
print( onehottocards(onehot) ) """