

import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers


import json
import os
import numpy as np




def get_decks():

    fobj = open("decksandpositions.json", "rb")
    data = json.load(fobj)


    length = len(data)

    #[main:  [deck: [cards:], [pos: ] ]  ]

    ohvlen = len( data[0][0] )


    decks = np.zeros((length, ohvlen))
    positions = np.zeros((length, 2))


    deckid = 0

    for deck in data:
        #print(deck)

        decks[deckid] = deck[0]

        positions[deckid] = deck[1]

        
        deckid +=1

    print(positions)


    return decks




model = keras.models.load_model('MODEL')



prediction = model.predict( get_decks() )


#print( prediction.shape )

f = open("predicted.json", "w")
f.write( json.dumps( prediction.tolist() )  )
f.close()



#print(prediction)