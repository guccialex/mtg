import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers


import json
import os
import numpy as np


def write_predictions_to_file(model):


    fobj = open("mldecks_no_mutations.json", "rb")
    data = json.load(fobj)
    length = len(data)

    decks = np.zeros((length, 1899))

    for deckid in range(length):

        for card in data[deckid][0]:

            cardid = card[0]
            cardscore = card[1]

            decks[deckid][cardid] = cardscore
    
    result =  model.predict(decks)

    f = open("prediction.json", "w")
    f.write( json.dumps( result.tolist() )  )
    f.close()



'''
import each deck and the position it should be going to

train it on that

use this to predict the position of each deck

write that to a file

use that file to suggest the next position


'''
                




def get_ml_decks_unmutated():


    fobj = open("mldecks_no_mutations.json", "rb")
    data = json.load(fobj)


    length = len(data)
    #length = 1000

    decks = np.zeros((length, 1899))

    positions = np.zeros( (length, 2) )


    for deckid in range(length):

        positions[deckid][0] = 1.0001
        positions[deckid][1] = 1.0001
        
        for card in data[deckid][0]:

            cardid = card[0]
            cardscore = card[1]

            decks[deckid][cardid] = cardscore
                
    return (decks , positions)



#get the 


(decks, positions) = get_ml_decks_unmutated()




inputs = keras.Input(shape=(1899))
m1 = layers.Dense(5, activation="relu") (inputs)
o = layers.Dense(2, activation="relu") (m1)
outputs = layers.Dense( 2 )(o)


model = keras.Model(inputs=inputs, outputs=outputs, name="mtgmodel")



model.compile(
    loss= keras.losses.MeanSquaredError(),
    optimizer=keras.optimizers.Adam(
        learning_rate=0.001,
        beta_1=0.9,
        beta_2=0.9,
        epsilon=0.001,
        #amsgrad=True,
        ),
    metrics=["accuracy"],
)




model.fit(decks, positions, batch_size=100, epochs=10)

write_predictions_to_file(model)

'''
prediction = model.predict(  np.zeros((1,1899)) )
print(prediction)
'''

#for each deck, predict it and write its predicted to a position



GCS_PATH_FOR_SAVED_MODEL = 'gs://vertex-ai-test-5/somename'
localmodel = 'TEMPMODEL'

model.save('MODEL')

